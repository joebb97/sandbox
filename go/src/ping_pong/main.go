package main

import (
	"fmt"
	"sync"
)

func mainChannels() {
	numTimes := 5
	var wg sync.WaitGroup
	theChannel := make(chan string)

	// Ping thread
	wg.Add(2)
	go func(n int) {
		defer wg.Done()
		for i := 0; i < n; i++ {
			fmt.Println(<-theChannel)
			theChannel <- "pong"
		}
	}(numTimes)

	// Pong thread
	go func(n int) {
		defer wg.Done()
		for i := 0; i < n; i++ {
			theChannel <- "ping"
			fmt.Println(<-theChannel)
		}
	}(numTimes)

	wg.Wait()
}

func mainMutex() {
	numTimes := 5
	var wg sync.WaitGroup
	m := sync.Mutex{}
	cv, printStr := sync.NewCond(&m), "ping"

	// Ping thread
	wg.Add(1)
	go func(n int) {
		defer wg.Done()
		for i := 0; i < n; i++ {
			cv.L.Lock()
			for printStr == "pong" {
				cv.Wait()
			}
			fmt.Println(printStr)
			printStr = "pong"
			cv.Signal()
			cv.L.Unlock()
		}
	}(numTimes)

	// Pong thread
	wg.Add(1)
	go func(n int) {
		defer wg.Done()
		for i := 0; i < n; i++ {
			cv.L.Lock()
			for printStr == "ping" {
				cv.Wait()
			}
			fmt.Println(printStr)
			printStr = "ping"
			cv.Signal()
			cv.L.Unlock()
		}
	}(numTimes)

	wg.Wait()
}

func main() {
	// mainChannels()
	mainMutex()
}
