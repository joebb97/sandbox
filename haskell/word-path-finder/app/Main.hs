module Main where

import System.Environment (getArgs)

main :: IO ()
main = do
    args <- getArgs
    putStrLn . unwords $ args
