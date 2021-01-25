module Board exposing (..)

import Dict exposing (Dict)
import Set exposing (Set)


rowSize =
    9


preSolved =
    17


type alias Tile =
    { value : String, possibleVals : Set Int }


defaultTile : Tile
defaultTile =
    { value = "", possibleVals = Set.empty }


type alias Board =
    Dict ( Int, Int ) Tile


defaultBoard : Board
defaultBoard =
    let
        tuples =
            getIndicesCat

        initVals =
            Set.fromList <| List.range 1 9

        tup_to_rec =
            \tup ->
                { value = ""
                , possibleVals = initVals
                }

        init_board =
            Dict.fromList <| List.map (\tup -> ( tup, tup_to_rec tup )) tuples
    in
    init_board


type alias UpdateBoardMsg =
    { rowID : Int, colID : Int, newValue : String, oldValue : String }


applyUpdate : UpdateBoardMsg -> Board -> Board
applyUpdate recMsg board =
    let
        tup =
            ( recMsg.rowID, recMsg.colID )

        the_rec =
            Maybe.withDefault defaultTile <| Dict.get tup board

        _ =
            Debug.log "applyUpdate" <| Debug.toString recMsg ++ Debug.toString the_rec

        newPossibleVals =
            if validValue recMsg.newValue then
                Set.empty

            else
                the_rec.possibleVals

        new_rec =
            { the_rec | value = recMsg.newValue, possibleVals = newPossibleVals }
    in
    Dict.insert tup new_rec board


applyUpdateAndFix : UpdateBoardMsg -> Board -> Board
applyUpdateAndFix recMsg board =
    fixPossibleVals recMsg <| applyUpdate recMsg board


adjustPossibleVal : UpdateBoardMsg -> Set Int -> Tile -> Tile
adjustPossibleVal recMsg cands tile =
    let
        newPossibleVals =
            if validValue recMsg.newValue then
                let
                    asInt =
                        Maybe.withDefault 0 <| String.toInt recMsg.newValue
                in
                Set.remove asInt tile.possibleVals

            else if validValue recMsg.oldValue then
                let
                    asInt =
                        Maybe.withDefault 0 <| String.toInt recMsg.oldValue
                in
                Set.union tile.possibleVals <| Set.insert asInt cands

            else
                tile.possibleVals
    in
    { tile | possibleVals = newPossibleVals }


fixPossibleVals : UpdateBoardMsg -> Board -> Board
fixPossibleVals recMsg board =
    let
        neighbors =
            Set.fromList <| List.map .value <| List.map (\tup -> getTile tup board) <| getNeighbors recMsg

        allPossible =
            Set.fromList <| List.map String.fromInt <| List.range 1 9

        cands  =
            Set.map
                (\item ->
                    Maybe.withDefault 0 <|
                        String.toInt item
                )
            <|
                Set.remove "" <|
                    Set.diff allPossible neighbors
                    
        updateTile =
            \tup ->
                ( tup
                , adjustPossibleVal recMsg cands <|
                    getTile tup board
                )

        otherDict =
            Dict.fromList <| List.map updateTile <| getNeighbors recMsg


        newBoard =
            Dict.union otherDict board

        -- _ =
        --     Debug.log "fixVals" <| Debug.toString otherDict
    in
    newBoard


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


getNeighbors : UpdateBoardMsg -> List ( Int, Int )
getNeighbors recMsg =
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
    in
    allCoords


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
