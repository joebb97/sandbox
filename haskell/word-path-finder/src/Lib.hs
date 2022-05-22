module Lib where

someFunc :: IO ()
someFunc = do
  l <- getContents
  print ("someFunct", l)
