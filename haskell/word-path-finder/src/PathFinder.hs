module PathFinder where

import qualified Data.Map as Map

buildPathFinder args = a
  where

findPath pathFinder = "theAnswer"

data PathFinder =
  PathFinder
    { beginWord :: String
    , endWord :: String
    , dict :: Map.Map Int String
    }
  deriving (Show)
-- let initPf = pushWord pf beginWord
-- let path = findPath initPf
--
-- findPath :: PathFinder -> [String]
-- findPath pathFinder = 
--   let top = popWord queue
--   if top == endWord then
--     stop
--     return path
--   else 
--     add children
--     recurse
