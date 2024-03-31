package playground

import "testing"

func TestSomething(t *testing.T) {
	s := Something{}
	if s.someField != 0 {
		t.Fatalf("oops")
	}
}
