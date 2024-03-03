main = mapM_ print $ stew 8 [1 .. 12]

data Result
  = Single String Int
  | Many String [Int]
  deriving (Show)

stew :: Int -> [Int] -> [Result]
stew num list =
  let filtered = filter (\n -> n < num && even n) list
      mapped = map (+ num) list
      folded = foldr (+) 0 list
      summed = sum list
      composedFunctions = product . map (+ num) $ list
   in [ Many "unaltered" list
      , Many "filtered" filtered
      , Many "mapped" mapped
      , Single "composed" composedFunctions
      , Single "folded" folded
      , Single "summed" summed
      ]
