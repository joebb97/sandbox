module Board exposing (..)

import Array exposing (Array)
import Dict exposing (Dict)


rowSize =
    9


type alias Tile =
    { value : String, rowID : Int, colID : Int, possibleVals : Array Int }


defaultTile : Tile
defaultTile =
    { value = "", rowID = -1, colID = -1, possibleVals = Array.empty }


type alias Board =
    Dict ( Int, Int ) Tile


defaultBoard : Board
defaultBoard =
    Dict.empty


type alias UpdateBoardMsg =
    { rowID : Int, colID : Int, newValue : String }


applyUpdate : UpdateBoardMsg -> Board -> Board
applyUpdate recMsg board =
    let
        tup =
            ( recMsg.rowID, recMsg.colID )

        the_rec =
            Maybe.withDefault defaultTile <| Dict.get tup board

        newPossibleVals =
            if validValue recMsg.newValue then
                Array.empty

            else
                the_rec.possibleVals
    in
    Dict.insert tup { the_rec | value = recMsg.newValue, possibleVals = newPossibleVals } board


fixPossibleVals : UpdateBoardMsg -> Board -> Board
fixPossibleVals recMsg board =
    -- This function is only called when validValue recMsg.newValue is true
    board


validValue tileVal =
    case String.toInt tileVal of
        Just value ->
            if value >= 1 && value <= 9 then
                True

            else
                False

        Nothing ->
            False


getIndicesCat : List ( Int, Int )
getIndicesCat =
    let
        zero_to_eight =
            List.range 0 8
    in
    List.concat <|
        List.map (\rowID -> List.map (\colID -> ( rowID, colID )) <| zero_to_eight) <|
            zero_to_eight


getIndices : List (List ( Int, Int ))
getIndices =
    let
        zero_to_eight =
            List.range 0 8
    in
    List.map (\rowID -> List.map (\colID -> ( rowID, colID )) <| zero_to_eight) <|
        zero_to_eight


getTile : ( Int, Int ) -> Board -> Tile
getTile tup board =
    Maybe.withDefault defaultTile <| Dict.get tup board
