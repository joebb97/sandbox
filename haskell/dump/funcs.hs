main = do
  putStrLn $ ourfunc 0
  putStrLn $ ourfunc 8
  putStrLn $ ourfunc (-1)
  -- A comment
  putStrLn $ ourfunc' 0
  putStrLn $ ourfunc' 8
  putStrLn $ ourfunc' (-1)

-- ourfunc :: Int -> String
ourfunc :: (Eq a, Num a) => a -> String
ourfunc 8 = "How ya going"
ourfunc 0 = "I'm not loud, I'm just cuban"
ourfunc num = "Spearfish, South Dakota"

ourfunc' :: (Eq a, Num a) => a -> String
ourfunc' num 
    | num == 8 = "How ya gaing"
    | num == 0 = "It's 58 outside, freezing!"
    | otherwise = "Spearfish, North Dakota"
