module BoardSolve exposing (..)

import Board exposing (..)
import Dict
import Set exposing (Set)


type SearchState
    = Halt
    | Continue
    | Success


tryValue : ( Int, Int, Int ) -> ( Board, SearchState ) -> ( Board, SearchState )
tryValue triple boardSolvedPair =
    let
        ( board, _ ) =
            boardSolvedPair

        ( possibleVal, row, col ) =
            triple

        tile =
            getTile ( row, col ) board

        updateMsg =
            { newValue = String.fromInt possibleVal
            , oldValue = tile.value
            , rowID = row
            , colID = col
            , newImmutable = False
            }

        ( newBoard, newState ) =
            solveBoard <| applyUpdateAndFix updateMsg board
    in
    ( newBoard, newState )


solveTile : ( Int, Int ) -> Tile -> ( Board, SearchState ) -> ( Board, SearchState )
solveTile key tile boardSolvedPair =
    let
        ( row, col ) =
            key

        ( board, searchState ) =
            boardSolvedPair

        ( newBoard, newSolved ) =
            if searchState == Halt then
                ( board, Halt )

            else if searchState == Success then
                ( board, Success )

            else if Set.isEmpty tile.possibleVals then
                if not (validValue tile.value) then
                    ( board, Halt )

                else
                    ( board, Continue )

            else
                let
                    asList =
                        Set.toList tile.possibleVals

                    outcomes =
                        List.map
                            (\possibleVal -> tryValue ( possibleVal, row, col ) boardSolvedPair)
                            asList

                    stillHope =
                        List.filter (\outcome -> Tuple.second outcome == Continue) outcomes

                    succeeded =
                        List.filter (\outcome -> Tuple.second outcome == Success) outcomes

                    failed =
                        List.filter (\outcome -> Tuple.second outcome == Halt) outcomes
                in
                if not (List.isEmpty succeeded) then
                    Maybe.withDefault ( board, Halt ) <| List.head succeeded

                else if not (List.isEmpty stillHope) then
                    Maybe.withDefault ( board, Halt ) <| List.head stillHope

                else
                    Maybe.withDefault ( board, Halt ) <| List.head failed
    in
    ( newBoard, newSolved )


isSolved : Board -> Bool
isSolved board =
    let
        helper tup solved =
            let
                tile =
                    getTile tup board
            in
            if validValue tile.value then
                True && solved

            else
                False
    in
    List.foldl helper True getIndicesCat


solveBoard : Board -> ( Board, SearchState )
solveBoard board =
    let
        ( newBoard, solved ) =
            if isSolved board then
                ( board, Success )

            else
                Dict.foldl solveTile ( board, Continue ) board
    in
    ( newBoard, solved )
