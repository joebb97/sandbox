module Util exposing (..)

import Board exposing (..)
import Dict


wikipediaBoard : Board
wikipediaBoard =
    let
        updates =
            [ ( 0, 0, 5 )
            , ( 0, 1, 3 )
            , ( 0, 4, 7 )
            , ( 1, 0, 6 )
            , ( 1, 3, 1 )
            , ( 1, 4, 9 )
            , ( 1, 5, 5 )
            , ( 2, 1, 9 )
            , ( 2, 2, 8 )
            , ( 2, 7, 6 )
            , ( 3, 0, 8 )
            , ( 3, 4, 6 )
            , ( 3, 8, 3 )
            , ( 4, 0, 4 )
            , ( 4, 3, 8 )
            , ( 4, 5, 3 )
            , ( 4, 8, 1 )
            , ( 5, 0, 7 )
            , ( 5, 4, 2 )
            , ( 5, 8, 6 )
            , ( 6, 1, 6 )
            , ( 6, 6, 2 )
            , ( 6, 7, 8 )
            , ( 7, 3, 4 )
            , ( 7, 4, 1 )
            , ( 7, 5, 9 )
            , ( 7, 8, 5 )
            , ( 8, 4, 8 )
            , ( 8, 7, 7 )
            , ( 8, 8, 9 )
            ]

        doUpdate update curBoard =
            let
                (row,col,value) = update
                asRec = { newValue = String.fromInt value, oldValue = "", rowID = row, 
                          colID = col, newImmutable = True }
            in
            applyUpdateAndFix asRec curBoard

        board = 
            List.foldl (doUpdate) defaultBoard updates
    in
    board
