module Main where

import System.Environment (getArgs)
import PathFinder

main :: IO ()
main = do
    args <- getArgs
    contents <- getContents
    let pf = buildPathFinder args contents
    print $ pf
    print $ findPath pf
