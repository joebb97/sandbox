package main

import (
	"bytes"
	"flag"
	"fmt"
	"io"
	"math"
	"net"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/pkg/errors"
)

const endbyte byte = 'E'

// Flags keeps track of the command line arguments
type Flags struct {
	addr, bufSize, port, duration, protoStr string
	isServer, isClient                      bool
}

// Open a connection to address addr, which is of format "<IP address>:port"
func Open(proto string, addr string) (net.Conn, error) {
	fmt.Println("Dialing " + addr + " using " + strings.ToUpper(proto))
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

func handleConnection(conn net.Conn, bufSize uint, protoStr string) {
	buf := make([]byte, bufSize)
	if protoStr == "tcp" {
		defer conn.Close()
	}
	var remoteAddr *net.UDPAddr
	totalRecvd := 0
	startTime := time.Now()
	var size int
	var err error
	for {
		if remoteAddr == nil && protoStr == "udp" {
			size, remoteAddr, err = conn.(*net.UDPConn).ReadFromUDP(buf)
			startTime = time.Now()
		} else {
			size, err = conn.Read(buf)
		}
		switch {
		case err == io.EOF:
			fmt.Println("Connection reached EOF, closing.\n ---")
			return
		case err != nil:
			fmt.Println("Error receiving message from connection\n", err)
			return
		}
		totalRecvd += size
		if buf[size-1] == endbyte {
			break
		}
	}
	duration := time.Since(startTime).Seconds()
	var addr interface{}
	if protoStr == "udp" {
		addr = remoteAddr
	} else {
		addr = conn.RemoteAddr()
	}
	kiloBytes := int(float64(totalRecvd) / math.Pow10(3))
	megaBitsPerSecond := (float64(totalRecvd) * 8.0 / math.Pow10(6)) / duration
	fmt.Printf("Received %+v KB in %+v seconds (rate=%.3f Mbps) from %+v\n",
		kiloBytes, duration, megaBitsPerSecond, addr)
}

func runServer(flags *Flags) error {
	bufSize, _ := strconv.Atoi(flags.bufSize)
	toUpper := strings.ToUpper(flags.protoStr)
	if flags.protoStr == "tcp" {
		listener, err := net.Listen(flags.protoStr, ":"+flags.port)
		if err != nil {
			return errors.Wrapf(err, "Unable to listen on port %s\n", flags.port)
		}
		fmt.Printf("Listening on %+v using %+v\n", listener.Addr().String(), toUpper)
		for {
			conn, err := listener.Accept()
			// fmt.Println("Accepted a connection request.")
			if err != nil {
				fmt.Println("Failed to accept a connection request:", err)
				continue
			}
			go handleConnection(conn, uint(bufSize), flags.protoStr)
		}
		// return nil
	} else if flags.protoStr == "udp" {
		// No sense of a connection in UDP, so handle each packet
		// individually.
		addr, err := net.ResolveUDPAddr(flags.protoStr, ":"+flags.port)
		if err != nil {
			return errors.Wrap(err, "Unable to listen resolve localhost network addr")
		}
		conn, err := net.ListenUDP(flags.protoStr, addr)
		if err != nil {
			return errors.Wrapf(err, "Unable to listen on port %s\n", flags.port)
		}
		fmt.Printf("Listening on %+v using %+v\n", conn.LocalAddr().String(), toUpper)
		for {
			handleConnection(conn, uint(bufSize), flags.protoStr)
		}

	} else {
		return errors.New("Unsupported protoStr")
	}
}

func runClient(flags *Flags) error {
	wholeAddr := flags.addr + ":" + flags.port
	conn, err := Open(flags.protoStr, wholeAddr)
	if err != nil {
		return err
	}
	defer conn.Close()
	bufSize, _ := strconv.Atoi(flags.bufSize)
	buf := bytes.Repeat([]byte{0}, bufSize)
	totalSent := 0

	duration, _ := strconv.Atoi(flags.duration)
	endTime := time.Now().Add(time.Second * time.Duration(duration))
	for time.Now().Before(endTime) {
		bytesSent, err := conn.Write(buf)
		if err != nil {
			return errors.Wrap(err, "Couldn't write message to server")
		}
		totalSent += bytesSent
	}
	buf[0] = endbyte
	if flags.protoStr == "tcp" {
		_, err = conn.Write(buf[:1])
		if err != nil {
			return errors.Wrap(err, "Couldn't write message to server")
		}
	} else if flags.protoStr == "udp" {
		_, err = conn.Write(buf[:1])
		if err != nil {
			return errors.Wrap(err, "Couldn't write message to server")
		}
		// TODO: Do more sophisticated connection closing
		// The ending packet that was just sent isn't guaranteed to make it
		// to the server

		// Server should only send back one byte END message
		// recvBuf := []byte{0}
		// for recvBuf[0] != endbyte {
		// 	_, err = conn.Read(recvBuf)
		// 	if err != nil {
		// 		return errors.Wrap(err, "Couldn't write message to server")
		// 	}
		// }

	}
	return nil
}

// Following tutorial from here: https://appliedgo.net/networking/
func main() {
	flags := Flags{}
	flag.StringVar(&flags.protoStr, "proto", "tcp", "Transport layer protocol to use")
	flag.StringVar(&flags.port, "port", "5001", "Port to listen on or connect to")
	flag.StringVar(&flags.addr, "a", "localhost", "Address or hostname of server to connect to.")
	flag.StringVar(&flags.bufSize, "b", "1470", "Size of buffer to use.")
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
