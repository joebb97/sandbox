module PathFinder where

import qualified Data.Map as Map
import qualified Data.Set as Set

buildPathFinder args contents = pf
  where
    pf = (asSet, contents)
    asSet = Set.fromList args

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
