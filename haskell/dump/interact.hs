main = 
    interact greet

greet allLines = 
    let
        asList = lines allLines
        greeted = map ("hello"++) asList

    in unlines greeted
