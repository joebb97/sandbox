module BoardGen exposing (..)

import Board exposing (..)
import Random
import Random.Set
import Set exposing (Set)


preSolved =
    27


toStart =
    10


coordGen : Random.Generator ( Int, Int )
coordGen =
    Random.pair (Random.int 0 8) (Random.int 0 8)


preFilledGen : Int -> Random.Generator (Set ( Int, Int ))
preFilledGen num =
    Random.Set.set num coordGen


addRandomTileAt : ( Int, Int ) -> Board -> Random.Generator Board
addRandomTileAt coord board =
    let
        theTile =
            getTile coord board

        ( row, col ) =
            coord

        asList =
            Set.toList theTile.possibleVals

        ( front, back ) =
            case asList of
                [] ->
                    ( -1, [] )

                head :: rest ->
                    ( head, rest )

        recMsg val =
            let
                rec =
                    { rowID = row
                    , colID = col
                    , newValue = val
                    , oldValue = theTile.value
                    , newImmutable = True
                    }

                -- _ =
                --     Debug.log "addRandom" <| "possible vals = " ++ Debug.toString asList ++ " " ++ Debug.toString rec
            in
            rec
    in
    Random.uniform front back
        |> Random.map (\sel -> applyUpdateAndFix (recMsg <| String.fromInt sel) board)


boardFromPositions : Board -> Set ( Int, Int ) -> Random.Generator Board
boardFromPositions board positions =
    -- let
    --     _ =
    --         Debug.log "boardFromPositions" <| Debug.toString positions ++ " " ++ Debug.toString (Set.size positions)
    -- in
    Set.foldl
        (\pos boardSoFarGen ->
            boardSoFarGen
                |> Random.andThen (addRandomTileAt pos)
        )
        (Random.constant board)
        positions


removeSome : Board -> Set ( Int, Int ) -> Board
removeSome board indices =
    let

        coordToRecMsg coord inBoard =
            let
                tile =
                    getTile coord inBoard

                ( row, col ) =
                    coord

                msg =
                    { rowID = row
                    , colID = col
                    , oldValue = tile.value
                    , newValue = ""
                    , newImmutable = False
                    }
            in
            msg

        helper theBoard theIndices =
            Set.foldl
                (\coord curBoard -> applyUpdateAndFix (coordToRecMsg coord curBoard) curBoard)
                theBoard
                theIndices

        newBoard =
            helper board indices
    in
    newBoard
