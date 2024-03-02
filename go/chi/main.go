package main

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"
)

// timeout expires the context of the request will be cancelled.
func RequestContextTimeout(duration time.Duration) func(http.Handler) http.Handler {

	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

			if duration > 0 {
				ctx, cancel := context.WithTimeout(r.Context(), duration)
				defer cancel()
				r = r.WithContext(ctx)
			}

			next.ServeHTTP(w, r)
		})
	}
}

func main() {
	r := chi.NewRouter()
	r.Use(RequestContextTimeout(300 * time.Millisecond))
	// handlers ...
	r.Get("/long", func(w http.ResponseWriter, r *http.Request) {
		// ctx := r.Context()

		// select {
		// case <-ctx.Done():
		// 	if err := ctx.Err(); err != nil {
		// 		fmt.Println("Timedout", err)
		// 	}
		// 	return

		// case <-time.After(time.Millisecond * 400):
		// 	// The above channel simulates some hard work.
		// }

		w.Write([]byte("done"))
	})
	http.ListenAndServe(":3000", r)
}
