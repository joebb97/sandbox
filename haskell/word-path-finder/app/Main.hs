module Main where

import System.Environment (getArgs)
import PathFinder

main :: IO ()
main = do
    args <- getArgs
    print $ buildPathFinder args
