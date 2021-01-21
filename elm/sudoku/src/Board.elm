module Board exposing (..)

import Array exposing (Array)


rowSize =
    9


type alias Tile =
    { value : String, rowID : Int, colID : Int, possibleVals : Array Int }


defaultTile : Tile
defaultTile =
    { value = "", rowID = -1, colID = -1, possibleVals = Array.empty }


type alias Board =
    Array (Array Tile)


type alias UpdateBoardMsg =
    { rowID : Int, colID : Int, newValue : String }


applyUpdate : UpdateBoardMsg -> Board -> Board
applyUpdate recMsg board =
    let
        the_row =
            Maybe.withDefault Array.empty <| Array.get recMsg.rowID board

        the_rec =
            Maybe.withDefault defaultTile <| Array.get recMsg.colID the_row

        newPossibleVals =
            if validValue recMsg.newValue then
                Array.empty

            else
                the_rec.possibleVals

        output =
            Array.set recMsg.colID { the_rec | value = recMsg.newValue, possibleVals = newPossibleVals } the_row
    in
    Array.set recMsg.rowID output board


fixPossibleVals : UpdateBoardMsg -> Board -> Board
fixPossibleVals recMsg board =
    -- This function is only called when validValue recMsg.newValue is true
    let
        the_row =
            Maybe.withDefault Array.empty <| Array.get recMsg.rowID board

        new_val =
            Maybe.withDefault -1 <| String.toInt recMsg.newValue

        new_row =
            Array.map (\rec -> { rec | possibleVals = Array.filter (\item -> item /= new_val) rec.possibleVals }) the_row
    in
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
