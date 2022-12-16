package main

import (
	"fmt"
	"math/rand"
	"net/http"
	"strings"
	"time"
)

type UserState struct {
	balance int
}

var Sessions map[string]*UserState = make(map[string]*UserState)

type AuthenticatedHandler = func(http.ResponseWriter, *http.Request, *UserState)

func balance(w http.ResponseWriter, _ *http.Request, user *UserState) {
	fmt.Println("balance is running")
	fmt.Fprintln(w, "User has balance of", user.balance)
}

func deposit(w http.ResponseWriter, r *http.Request, user *UserState) {
	if r.Method == "GET" {
		html := `
	<!DOCTYPE html>
	<html>
	<form action="/deposit" method="post">
	  <label for="data">data:</label><br>
	  <input type="text" id="fname" name="fname"><br>
	  <input type="submit" value="Submit">
	</form>
	</body>
	</html>
		`
		fmt.Fprintln(w, html)
		return
	}
	user.balance -= 300
	fmt.Fprintln(w, "After deposit User has balance of", user.balance)
}

func ensureCookie(next AuthenticatedHandler) http.HandlerFunc {
	fmt.Println("ensureCookie called")
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html")
		if existingCookie, err := r.Cookie("SessionID"); err == nil {
			cookieVal := existingCookie.Value
			var user *UserState
			var ok bool
			if user, ok = Sessions[cookieVal]; !ok {
				w.WriteHeader(403)
				fmt.Fprintf(w, "You sent a cookie, but I didn't give it to you!\n")
				return
			}
			fmt.Fprintf(w, "You already have a cookie %s , and I gave it to you\n", cookieVal)
			next(w, r, user)
		} else {
			fmt.Println("We redirected to / ")
			http.Redirect(w, r, "/", 302)
		}

	}
}

func login(w http.ResponseWriter, r *http.Request) {
	fmt.Println("Login is running")
	if existingCookie, err := r.Cookie("SessionID"); err == nil {
		cookieVal := existingCookie.Value
		if _, ok := Sessions[cookieVal]; !ok {
			w.WriteHeader(403)
			fmt.Fprintf(w, "You sent a cookie, but I didn't give it to you!\n")
			return
		}
		fmt.Fprintf(w, "You already have a cookie %s , and I gave it to you\n", cookieVal)
		return
	}
	cookie := &http.Cookie{
		Name:  "SessionID",
		Value: GenerateRandString(22),
		Path:  "/",
	}
	http.SetCookie(w, cookie)
	fmt.Fprintf(w, "You have no cookie\n")
	fmt.Fprintf(w, "Set your cookie to %s", cookie)
	Sessions[cookie.Value] = &UserState{balance: 1000}
	fmt.Fprintf(w, "\n%s\n", *r)
}

func notFound(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprint(w, "suh dude, no page here")
}

func main() {
	http.Handle("/favicon.ico", http.NotFoundHandler())
	http.HandleFunc("/", login)
	http.HandleFunc("/balance", ensureCookie(balance))
	http.HandleFunc("/deposit", ensureCookie(deposit))
	http.ListenAndServe(":8088", nil)
}

func GenerateRandString(length int) string {
	r := rand.New(rand.NewSource(time.Now().Unix()))
	bytes := make([]byte, length)
	for i := 0; i < length; i++ {
		b := r.Intn(26) + 65
		bytes[i] = byte(b)
	}
	return strings.ToLower(string(bytes))
}
