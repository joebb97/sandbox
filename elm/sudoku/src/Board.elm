module Board exposing (..)

import Dict exposing (Dict)


rowSize =
    9


type alias Tile =
    { value : String, rowID : Int, colID : Int, possibleVals : List Int }


defaultTile : Tile
defaultTile =
    { value = "", rowID = -1, colID = -1, possibleVals = [] }


type alias Board =
    Dict ( Int, Int ) Tile


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
                []

            else
                the_rec.possibleVals

        new_rec =
            { the_rec | value = recMsg.newValue, possibleVals = newPossibleVals }
    in
    Dict.insert tup new_rec board


ruleOut : Tile -> Int -> Tile
ruleOut tile val =
    let
        newPossibleVals =
            tile.possibleVals
    in
    tile


quadrantCoords : ( Int, Int ) -> List ( Int, Int )
quadrantCoords tup =
    let
        ( row, col ) =
            tup

        newRow =
            (row // 3) * 3

        newCol =
            (col // 3) * 3

        rowIndices =
            List.range newRow <| newRow + 2

        colIndices =
            List.range newCol <| newCol + 2

        quadrantIndices =
            List.filter (\tupIt -> tupIt /= tup) <|
                List.concat <|
                    List.map (\rowIt -> List.map (\colIt -> ( rowIt, colIt )) colIndices) rowIndices
    in
    quadrantIndices


fixPossibleVals : UpdateBoardMsg -> Board -> Board
fixPossibleVals recMsg board =
    -- This function is only called when validValue recMsg.newValue is true
    let
        indices =
            getIndicesCat

        sameRow =
            List.filter (\tup -> Tuple.first tup == recMsg.rowID) indices

        sameCol =
            List.filter (\tup -> Tuple.second tup == recMsg.colID) indices

        sameQuad =
            quadrantCoords ( recMsg.rowID, recMsg.colID )

        allCoords =
            List.concat [ sameRow, sameCol, sameQuad ]

        _ =
            Debug.log "fixVals" <| Debug.toString allCoords
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
