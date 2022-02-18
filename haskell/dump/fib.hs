main = print . nthFib $ 3000

nthFib n = last . take (n + 1) $ goodfib [0, 1]

goodfib memo@(x:xs:_) = x : goodfib (xs : [x + xs])

badfib 1 = 0
badfib 2 = 1
badfib n = sum . map badfib $ [backTwo, backOne]
  where
    backTwo = n - 2
    backOne = n - 1

badfib' 1 = 0
badfib' 2 = 1
badfib' n =
  let backTwo = n - 2
      backOne = n - 1
   in sum . map badfib' $ [backTwo, backOne]
