package main

import (
	"bufio"
	"flag"
	"io"
	"log"
	"net"
	"os"
	"strconv"
	"strings"

	"github.com/pkg/errors"
)

// Flags keeps track of the command line arguments
type Flags struct {
	addr, bufSize, port, duration, protoStr string
	isServer, isClient                      bool
}

// Open a connection to address addr, which is of format "<IP address>:port"
func Open(proto string, addr string) (net.Conn, error) {
	log.Println("Dialing " + addr + " using " + proto)
	conn, err := net.Dial(proto, addr)
	if err != nil {
		return nil, errors.Wrap(err, "Dialing "+addr+" failed")
	}
	return conn, nil
}

// CheckFlags validates the commandline arguments
// Returns nil if everything is okay, other throws returns an error
func CheckFlags(flags *Flags) error {
	if !(flags.isServer || flags.isClient) || (flags.isServer && flags.isClient) {
		return errors.New("Exactly one of -s or -c must be specified")
	}
	return nil
}

func handleConnection(conn net.Conn, bufSize uint) {
	buf := make([]byte, bufSize)
	defer conn.Close()
	for {
		size, err := conn.Read(buf)
		switch {
		case err == io.EOF:
			log.Println("Connection reached EOF, closing.\n ---")
			return
		case err != nil:
			log.Println("Error receiving message from connection\n", err)
			return
		}
		msg := string(buf[:size])
		msg = strings.Trim(msg, "\n")
		log.Println(msg)
	}
}

func runServer(flags *Flags) error {
	bufSize, _ := strconv.Atoi(flags.bufSize)
	if flags.protoStr == "tcp" {
		listener, err := net.Listen(flags.protoStr, ":"+flags.port)
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
	} else if flags.protoStr == "udp" {
		// No sense of a connection in UDP, so handle each packet
		// individually.
		addr, err := net.ResolveUDPAddr(flags.protoStr, ":"+flags.port)
		if err != nil {
			return errors.Wrap(err, "Unable to listen resolve localhost network addr")
		}
		listener, err := net.ListenUDP(flags.protoStr, addr)
		if err != nil {
			return errors.Wrapf(err, "Unable to listen on port %s\n", flags.port)
		}
		log.Println("Listening on ", listener.LocalAddr().String()+" using udp")
		buf := make([]byte, bufSize)
		for {
			size, _ := listener.Read(buf)
			msg := string(buf[:size])
			msg = strings.Trim(msg, "\n")
			log.Println(msg)
		}

	} else {
		return errors.New("Unsupported protoStr")
	}
}

func runClient(flags *Flags) error {
	wholeAddr := flags.addr + ":" + flags.port
	conn, err := Open(flags.protoStr, wholeAddr)
	if err != nil {
		return errors.Wrap(err, "Client: Failed to open connection to "+wholeAddr)
	}
	stdinReader := bufio.NewReader(os.Stdin)
	for {
		text, stdinErr := stdinReader.ReadString('\n')
		if stdinErr != nil {
			return errors.Wrap(stdinErr, "Couldn't read from stdin")
		}
		_, err := conn.Write([]byte(text))
		if err != nil {
			return errors.Wrap(err, "Couldn't write message to server")
		}
	}
	// return nil
}

// Following tutorial from here: https://appliedgo.net/networking/
func main() {
	flags := Flags{}
	flag.StringVar(&flags.protoStr, "proto", "tcp", "Transport layer protocol to use")
	flag.StringVar(&flags.port, "port", "5001", "Port to listen on or connect to")
	flag.StringVar(&flags.addr, "a", "localhost", "Address or hostname of server to connect to.")
	flag.StringVar(&flags.bufSize, "b", "1024", "Size of buffer to use.")
	flag.StringVar(&flags.duration, "t", "10", "Duration to run client for")
	flag.BoolVar(&flags.isServer, "s", false, "Whether this iperf is a server")
	flag.BoolVar(&flags.isClient, "c", false, "Whether this iperf is a client")

	flag.Parse()
	err := CheckFlags(&flags)
	if err != nil {
		log.Println(err)
		flag.Usage()
		os.Exit(1)
	}
	if flags.isServer {
		err := runServer(&flags)
		if err != nil {
			log.Println("Error:", errors.WithStack(err))
		}
	} else {
		err := runClient(&flags)
		if err != nil {
			log.Println("Error:", errors.WithStack(err))
		}
	}
}
