module BoardSolve exposing (..)

import Board exposing (..)
import Dict
import List.Extra
import Set exposing (Set)


type SearchState
    = Halt
    | Continue
    | Success


tryValue : ( Int, Int, Int ) -> ( Board, SearchState ) -> ( Board, SearchState )
tryValue triple boardSolvedPair =
    let
        ( board, curState ) =
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


solveTile : ( Int, Int ) -> ( Board, SearchState ) -> ( Board, SearchState )
solveTile coord boardSolvedPair =
    let
        ( row, col ) =
            coord

        ( board, searchState ) =
            boardSolvedPair

        tile =
            getTile coord board

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
                    -- TODO: Need to process all continues in case they yield success
                    Maybe.withDefault ( board, Halt ) <| List.head stillHope

                else
                    Maybe.withDefault ( board, Halt ) <| List.head failed
    in
    ( newBoard, newSolved )


solveBoard : Board -> ( Board, SearchState )
solveBoard board =
    let
        helper tup =
            let
                tile =
                    getTile tup board
            in
            not (validValue tile.value)

        unsolved =
            List.filter helper <| getIndicesCat

        ( newBoard, solved ) =
            if List.isEmpty unsolved then
                ( board, Success )

            else
                let
                    mrv =
                        List.Extra.minimumBy (\tup -> Set.size <| .possibleVals <| getTile tup board) unsolved

                    theHead =
                        Maybe.withDefault ( -1, -1 ) <| mrv
                in
                solveTile theHead ( board, Continue )
    in
    ( newBoard, solved )
