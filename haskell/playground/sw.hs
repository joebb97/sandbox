main = do
    print $ hasTargetWindow [1,3,8,10] 4
    print $ hasTargetWindow [10] 10
    print $ hasTargetWindow [0, 10] 10
    print $ hasTargetWindow [0] 10

hasTargetWindow arr target = (tup, rw == target)
    where
        arrLen = length arr
        (left, right, wsum) = (-1, -1, 0)
        tup@(rl, rr, rw) = helper (left, right, wsum) arr

        helper tup [] = tup
        helper tup@(left, right, wsum) arr
            | wsum == target = (left, right, wsum)
            | right + 1 >= arrLen || left + 1 >= arrLen = tup
            | wsum < target = 
                helper (left, right + 1, wsum + ((!!) arr $ right + 1)) arr
            | wsum > target = 
                helper (left + 1, right, wsum - ((!!) arr $ left + 1)) arr

