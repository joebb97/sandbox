package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"log"
	"net"
	"os"
	"strconv"

	"github.com/pkg/errors"
)

const quitMsg = "quit"
const shutdownMsg = "I'm finished"
const shutdownAckMsg = "I acknowledge you're finished"
const evenMore = "EVEN MORE"

// Flags keeps track of the command line arguments
type Flags struct {
	addr, bufSize, port, duration, protoStr string
	isServer, isClient, halfClose, evenMore bool
}

// Open a connection to address addr, which is of format "<IP address>:port"
func Open(proto string, addr string) (*net.TCPConn, error) {
	log.Println("Dialing " + addr + " using " + proto)
	conn, err := net.Dial(proto, addr)
	if err != nil {
		return nil, errors.Wrap(err, "Dialing "+addr+" failed")
	}
	return conn.(*net.TCPConn), nil
}

// CheckFlags validates the commandline arguments
// Returns nil if everything is okay, other throws returns an error
func CheckFlags(flags *Flags) error {
	if !(flags.isServer || flags.isClient) || (flags.isServer && flags.isClient) {
		return errors.New("Exactly one of -s or -c must be specified")
	}
	return nil
}

func getMsg(conn net.Conn, buf []byte) (string, error) {
	size, err := conn.Read(buf)
	switch {
	case size == 0 && err != nil:
		return "", nil
	case err == io.EOF:
		return "", fmt.Errorf("Connection reached EOF, closing.")
	case err != nil:
		str := "Error receiving message from connection %s."
		return "", fmt.Errorf(str, err)
	}
	msg := string(buf[:size])
	fmt.Println("got> " + msg)
	if msg == shutdownMsg {
		conn.Write([]byte(shutdownAckMsg))
		if flags.halfClose {
			conn.(*net.TCPConn).CloseWrite()
		}
		return shutdownAckMsg, nil
	}
	conn.Write(buf[:size])
	return msg, nil
}

func handleConnection(conn net.Conn, bufSize uint) {
	buf := make([]byte, bufSize)
	defer conn.Close()
	defer fmt.Println("---")
	defer log.Println("Server closing conn from " + conn.RemoteAddr().String())
	for {
		msg, err := getMsg(conn, buf)
		if err != nil {
			log.Println(err)
			return
		}
		if msg == "" {
			return
		}
	}
}

func runServer() error {
	bufSize, _ := strconv.Atoi(flags.bufSize)
	listener, err := net.Listen("tcp", ":"+flags.port)
	if err != nil {
		return errors.Wrapf(err, "Unable to listen on port %s\n", flags.port)
	}
	log.Println("Listening on", listener.Addr().String()+" using tcp")
	for {
		conn, err := listener.Accept()
		log.Println("Accepted a connection request.")
		if err != nil {
			log.Println("Failed to accept a connection request:", err)
			continue
		}
		go handleConnection(conn, uint(bufSize))
	}
	// return nil
}

func runClient() error {
	wholeAddr := flags.addr + ":" + flags.port
	conn, err := Open("tcp", wholeAddr)
	if err != nil {
		return errors.Wrap(err, "Client: Failed to open connection to "+wholeAddr)
	}
	defer conn.Close()
	stdinReader := bufio.NewReader(os.Stdin)
	bufSize, _ := strconv.Atoi(flags.bufSize)
	buf := make([]byte, uint(bufSize))
	for {
		fmt.Print("send> ")
		text, stdinErr := stdinReader.ReadString('\n')
		if stdinErr != nil {
			return errors.Wrap(stdinErr, "Couldn't read from stdin")
		}
		text = text[:len(text)-1]
		if len(text) == 0 {
			continue
		}
		asByte := []byte(text)
		_, err := conn.Write(asByte)
		if err != nil {
			return errors.Wrap(err, "Couldn't write message to server")
		}
		replySize, err := conn.Read(buf)
		if err != nil {
			return errors.Wrap(err, "Couldn't read reply from server")
		}
		// log.Println(replySize, err)
		// Don't print the newline
		msg := string(buf[:replySize])
		fmt.Println("got> " + msg)
		if text == quitMsg {
			log.Println("Client exiting")
			break
		}
	}
	_, err = conn.Write([]byte(shutdownMsg))
	if err != nil {
		return errors.Wrap(err, "Couldn't write message to server")
	}
	if flags.halfClose {
		conn.CloseWrite()
	}
	replySize, err := conn.Read(buf)
	// log.Println(replySize, err)
	// Don't print the newline
	msg := string(buf[:replySize])
	log.Println("got shutdown ack from server: msg =", msg, ", err =", err)
	if flags.evenMore {
		_, err = conn.Write([]byte(evenMore))
		if err != nil {
			return errors.Wrap(err, "Couldn't write message to server")
		}
	}
	return nil
}

var flags Flags

// Following tutorial from here: https://appliedgo.net/networking/
func main() {
	flags = Flags{}
	flag.StringVar(&flags.port, "port", "5001", "Port to listen on or connect to")
	flag.StringVar(&flags.addr, "a", "localhost", "Address or hostname of server to connect to.")
	flag.StringVar(&flags.bufSize, "b", "1024", "Size of buffer to use.")
	flag.StringVar(&flags.duration, "t", "10", "Duration to run client for")
	flag.BoolVar(&flags.isServer, "s", false, "Whether this is an echo server")
	flag.BoolVar(&flags.isClient, "c", false, "Whether this is an echo client")
	flag.BoolVar(&flags.halfClose, "y", false, "Whether this client or server should do a half close")
	flag.BoolVar(&flags.evenMore, "e", false, "Whether this client should send even more after reading the shutdown ack")

	flag.Parse()
	err := CheckFlags(&flags)
	if err != nil {
		log.Println(err)
		flag.Usage()
		os.Exit(1)
	}
	if flags.isServer {
		err := runServer()
		if err != nil {
			log.Println("Error:", errors.WithStack(err))
		}
	} else {
		err := runClient()
		if err != nil {
			log.Println("Error:", errors.WithStack(err))
		}
	}
}
