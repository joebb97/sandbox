main = print sol

-- List the first 20 pairs of numbers
--  where the product is:
--      * A palindrome of length greater than 2
--      * Includes the digits 0 and 9
--
--  Optional, but strongly encouraged type annotation
-- sol :: [(Int,Int,Int)]
sol =
  take
    numberToTake
    [ (x, y, prod)
    | u <- [0 ..]
    , x <- [0 .. u]
    , y <- [x .. u]
    , x + y == u
    -- let expressions allow you to bind variables
    -- almost anywhere! They can also produce values
    -- using `let <code> in`
    , let prod = x * y
    , let str = show prod
    , isPalindrome str
    , length str > 2
    , any (== '0') str && any (== '9') str
    ]
  -- the where clause allows you to define other 
  -- helpers / tmp variables used in the fuction
  where
    -- Again optional annotation
    -- but also good in helpers!
    isPalindrome :: String -> Bool
    isPalindrome str = str == reverse str

    numberToTake = 20
