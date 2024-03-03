#lang racket

(define (ishello str)
  (if (string-prefix? str "hello")
      (substring str 4 7)
      8
   ))

(ishello "ellowingdingwewewe")
