package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"net"
	"os"

	"github.com/pkg/errors"
)

// Flags keeps track of the command line arguments
type Flags struct {
	addr, port, duration string
	isServer, isClient   bool
	useUDP, useTCP       bool
}

// Open a connection to address addr
func Open(addr string) (*bufio.ReadWriter, error) {
	log.Println("Dial" + addr)
	conn, err := net.Dial("tcp", addr)
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
	// theip := net.ParseIP("192.0.0.-1")
	// theip == nil
	return nil
}

// Following tutorial from here: https://appliedgo.net/networking/
func main() {
	flags := Flags{useTCP: true}
	flag.StringVar(&flags.port, "p", ":0", "Port to listen on or connect to")
	flag.StringVar(&flags.port, "t", "10", "Duration to run client for")
	flag.BoolVar(&flags.isServer, "s", false, "Whether this iperf is a server")
	flag.BoolVar(&flags.isClient, "c", false, "Whether this iperf is a client")
	log.Printf("%v", os.Args)

	flag.Parse()
	log.Printf("%v", flags)
	err := CheckFlags(&flags)
	if err != nil {
		fmt.Println(err)
		flag.Usage()
		os.Exit(1)
	}
	if flags.isServer {
		// runServer()
	} else {
		// runClient()
	}
}
