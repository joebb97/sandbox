package main

import (
	"bufio"
	"flag"
	"io"
	"fmt"
	"net"
	"os"
	"strings"

	"github.com/pkg/errors"
)

// Flags keeps track of the command line arguments
type Flags struct {
	addr, port, duration, protoStr string
	isServer, isClient             bool
}

// Open a connection to address addr, which is of format "<IP address>:port"
func Open(proto string, addr string) (*bufio.ReadWriter, error) {
	conn, err := net.Dial(proto, addr)
	if err != nil {
		return nil, errors.Wrap(err, "Dialing "+addr+" failed")
	}
	return bufio.NewReadWriter(bufio.NewReader(conn), bufio.NewWriter(conn)), nil
}

// CheckFlags validates the commandline arguments
// Returns nil if everything is okay, other throws returns an error
func CheckFlags(flags *Flags) error {
	if !(flags.isServer || flags.isClient) || (flags.isServer && flags.isClient) {
		return errors.New("Exactly one of -s or -c must be specified")
	}
	return nil
}

func handleConnection(conn net.Conn) {
	rw := bufio.NewReadWriter(bufio.NewReader(conn), bufio.NewWriter(conn))
	defer conn.Close()
	for {
		msg, err := rw.ReadString('\n')
		switch {
		case err == io.EOF:
			fmt.Println("Connection reached EOF, closing.\n ---")
			return
		case err != nil:
			fmt.Println("Error receiving message from connection\n", err)
			return
		}
		msg = strings.Trim(msg, "\n ")
		fmt.Println(msg)
	}
}

func runServer(flags *Flags) error {
	if flags.protoStr == "tcp" {
		listener, err := net.Listen(flags.protoStr, ":"+flags.port)
		if err != nil {
			return errors.Wrapf(err, "Unable to listen on port %s\n", flags.port)
		}
		for {
			conn, err := listener.Accept()
			if err != nil {
				fmt.Println("Failed to accept a connection request:", err)
				continue
			}
			go handleConnection(conn)
		}
		// return nil
	} else if flags.protoStr == "udp" {
		addr, err := net.ResolveUDPAddr(flags.protoStr, ":"+flags.port)
		if err != nil {
			return errors.Wrap(err, "Unable to listen resolve localhost network addr")
		}
		listener, err := net.ListenUDP(flags.protoStr, addr)
		if err != nil {
			return errors.Wrapf(err, "Unable to listen on port %s\n", flags.port)
		}
		rw := bufio.NewReadWriter(bufio.NewReader(listener), bufio.NewWriter(listener))
		for {
			msg, _ := rw.ReadString('\n')	
			msg = strings.Trim(msg, "\n ")
			fmt.Println(msg)
		}

	} else {
		return errors.New("Unsupported protoStr")
	}
	return nil
}

func runClient(flags *Flags) error {
	wholeAddr := flags.addr + ":" + flags.port
	rw, err := Open(flags.protoStr, wholeAddr)
	if err != nil {
		return errors.Wrap(err, "Client: Failed to open connection to "+wholeAddr)
	}
	stdinReader := bufio.NewReader(os.Stdin)
	for {
		text, stdinErr := stdinReader.ReadString('\n')
		if stdinErr != nil {
			return errors.Wrap(stdinErr, "Couldn't read from stdin")
		}
		_, err := rw.WriteString(text)
		if err != nil {
			return errors.Wrap(err, "Couldn't write message to server")
		}
		rw.Flush()
	}
	// return nil
}

// Following tutorial from here: https://appliedgo.net/networking/
func main() {
	flags := Flags{}
	flag.StringVar(&flags.protoStr, "proto", "tcp", "Transport layer protocol to use")
	flag.StringVar(&flags.port, "port", "5001", "Port to listen on or connect to")
	flag.StringVar(&flags.addr, "a", "localhost", "Address or hostname of server to connect to.")
	flag.StringVar(&flags.duration, "t", "10", "Duration to run client for")
	flag.BoolVar(&flags.isServer, "s", false, "Whether this iperf is a server")
	flag.BoolVar(&flags.isClient, "c", false, "Whether this iperf is a client")

	flag.Parse()
	err := CheckFlags(&flags)
	if err != nil {
		fmt.Println(err)
		flag.Usage()
		os.Exit(1)
	}
	if flags.isServer {
		err := runServer(&flags)
		if err != nil {
			fmt.Println("Error:", errors.WithStack(err))
		}
	} else {
		err := runClient(&flags)
		if err != nil {
			fmt.Println("Error:", errors.WithStack(err))
		}
	}
}
