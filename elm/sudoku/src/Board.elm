module Board exposing (..)

import Dict exposing (Dict)
import Random
import Random.Set
import Set exposing (Set)


rowSize =
    9


preSolved =
    27


type alias Tile =
    { value : String, possibleVals : Set Int, immutable : Bool }


defaultTile : Tile
defaultTile =
    { value = "", possibleVals = Set.empty, immutable = False }


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
                , immutable = False
                }

        init_board =
            Dict.fromList <| List.map (\tup -> ( tup, tup_to_rec tup )) tuples
    in
    init_board


type alias UpdateBoardMsg =
    { rowID : Int, colID : Int, newValue : String, oldValue : String, newImmutable: Bool}


coordGen : Random.Generator ( Int, Int )
coordGen =
    Random.pair (Random.int 0 8) (Random.int 0 8)


preFilledGen : Random.Generator (Set ( Int, Int ))
preFilledGen =
    Random.Set.set preSolved coordGen


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


applyUpdate : UpdateBoardMsg -> Board -> Board
applyUpdate recMsg board =
    let
        tup =
            ( recMsg.rowID, recMsg.colID )

        the_rec =
            getTile tup board

        possibleVal =
            Maybe.withDefault 0 <| String.toInt recMsg.newValue

        ( newVal, newPossibleVals ) =
            if validValue recMsg.newValue && Set.member possibleVal the_rec.possibleVals then
                ( recMsg.newValue, Set.empty )

            else
                ( "", the_rec.possibleVals )

        new_rec =
            { the_rec | value = newVal, possibleVals = newPossibleVals, immutable = recMsg.newImmutable }
    in
    Dict.insert tup new_rec board


applyUpdateAndFix : UpdateBoardMsg -> Board -> Board
applyUpdateAndFix recMsg board =
    let
        tile =
            getTile ( recMsg.rowID, recMsg.colID ) board

        _ =
            Debug.log "applyUpdateAndFix" <| Debug.toString recMsg ++ " " ++ Debug.toString tile.possibleVals
    in
    fixPossibleVals recMsg <| applyUpdate recMsg board


adjustPossibleVal : UpdateBoardMsg -> Set Int -> ( Int, Int ) -> Board -> Tile
adjustPossibleVal recMsg cands coord board =
    let
        tile =
            getTile coord board

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

                    neighborVals =
                        getNeighborVals coord board
                in
                Set.intersect neighborVals <| Set.union tile.possibleVals <| Set.insert asInt cands

            else
                tile.possibleVals
    in
    { tile | possibleVals = newPossibleVals }


getNeighborVals : ( Int, Int ) -> Board -> Set Int
getNeighborVals coord board =
    let
        neighbors =
            Set.fromList <| List.map .value <| List.map (\tup -> getTile tup board) <| getNeighbors coord

        allPossible =
            Set.fromList <| List.map String.fromInt <| List.range 1 9

        cands =
            Set.map
                (\item ->
                    Maybe.withDefault 0 <|
                        String.toInt item
                )
            <|
                Set.remove "" <|
                    Set.diff allPossible neighbors
    in
    cands


fixPossibleVals : UpdateBoardMsg -> Board -> Board
fixPossibleVals recMsg board =
    let
        coord =
            ( recMsg.rowID, recMsg.colID )

        cands =
            getNeighborVals coord board

        updateTile =
            \tup ->
                ( tup
                , adjustPossibleVal recMsg cands tup board
                )

        otherDict =
            Dict.fromList <| List.map updateTile <| getNeighbors coord

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


getNeighbors : ( Int, Int ) -> List ( Int, Int )
getNeighbors coord =
    let
        ( row, col ) =
            coord

        indices =
            getIndicesCat

        sameRow =
            List.filter (\tup -> Tuple.first tup == row) indices

        sameCol =
            List.filter (\tup -> Tuple.second tup == col) indices

        sameQuad =
            quadrantCoords coord

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
    case Dict.get tup board of
        Just tile ->
            tile

        Nothing ->
            let
                _ =
                    Debug.log "DEFAULT TILE!!!!"
            in
            defaultTile
