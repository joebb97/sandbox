main = do
    putStr "name> "
    name <- getLine
    if name /= "q"
    then do
        putStrLn $ "hello " ++ name
        main
    else do
        return ()
