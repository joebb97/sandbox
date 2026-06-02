package main

import "fmt"

func main() {
    a := []rune{'0', '2', '4', '6', '⌘'}
    b := a[0:2]
    b[0] = 9
    fmt.Println(a, b)
}
