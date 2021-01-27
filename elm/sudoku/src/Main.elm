module Main exposing (..)

import Board exposing (..)
import Browser
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Random
import Set exposing (Set)
import Util exposing (..)



-- MAIN


type alias Model =
    { board : Board
    , solved : Bool
    }


main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = \_ -> Sub.none
        }


initialModel : Model
initialModel =
    { board = defaultBoard, solved = False }


init : () -> ( Model, Cmd Msg )
init _ =
    ( initialModel, Cmd.none )


type Msg
    = UpdateBoard UpdateBoardMsg
    | ClearBoard
    | GenerateBoard Int
    | NewRandom Board
    | SolveBoard


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    -- let
    --     _ =
    --         Debug.log "msg" <| Debug.toString msg
    -- in
    case msg of
        UpdateBoard recMsg ->
            let
                cleared =
                    { recMsg | newValue = "" }

                tile =
                    getTile ( recMsg.rowID, recMsg.colID ) model.board

                _ =
                    Debug.log "UpdateBoard Tile" <| Debug.toString tile

                newBoard =
                    case String.toInt recMsg.newValue of
                        Just value ->
                            if (value >= 1 && value <= 9) && Set.member value tile.possibleVals then
                                applyUpdateAndFix recMsg model.board

                            else
                                model.board

                        Nothing ->
                            fixPossibleVals cleared <| applyUpdate cleared model.board
            in
            if not recMsg.newImmutable then
                ( { board = newBoard, solved = False }, Cmd.none )

            else
                ( model, Cmd.none )

        ClearBoard ->
            ( initialModel, Cmd.none )

        GenerateBoard _ ->
            ( model
            , Random.generate
                NewRandom
                (preFilledGen |> Random.andThen (boardFromPositions defaultBoard))
            )

        NewRandom newRand ->
            ( { model | board = newRand }, Cmd.none )

        SolveBoard ->
            let
                ( newBoard, solved ) =
                    solveBoard model.board
            in
            ( { model | board = newBoard, solved = solved }, Cmd.none )


tryValue : Int -> ( Tile, Bool ) -> ( Tile, Bool )
tryValue possibleVal tileSolvedPair =
    let
        ( tile, solved ) =
            tileSolvedPair
    in
    ( tile, solved )


solveTile : ( Int, Int ) -> Tile -> ( Board, Bool ) -> ( Board, Bool )
solveTile key tile boardSolvedPair =
    let
        ( row, col ) =
            key

        ( board, solved ) =
            boardSolvedPair

        ( newBoard, newSolved ) =
            if Set.isEmpty tile.possibleVals then
                if not (validValue tile.value) then
                    ( board, False )

                else
                    ( board, solved )

            else if solved then
                ( board, True )

            else
                let
                    ( newValue, success ) =
                        Set.foldl tryValue ( tile, False ) tile.possibleVals

                    updateMsg =
                        { newValue = newValue.value
                        , oldValue = tile.value
                        , rowID = row
                        , colID = col
                        , newImmutable = False
                        }
                in
                ( applyUpdateAndFix updateMsg board, success )
    in
    ( newBoard, newSolved )


solveBoard : Board -> ( Board, Bool )
solveBoard board =
    let
        ( newBoard, solved ) =
            Dict.foldl solveTile ( Dict.empty, False ) board
    in
    ( newBoard, False )


immutableClassStr: Tile -> String
immutableClassStr tile =
    if tile.immutable then
        "immutable"
    else
        "mutable"

tileToInput : ( ( Int, Int ), Tile ) -> Html Msg
tileToInput info =
    let
        ( ( row, col ), tile ) =
            info

        boardMsg =
            { rowID = row
            , colID = col
            , oldValue = tile.value
            , newValue = tile.value
            , newImmutable = tile.immutable
            }

        helper input =
            UpdateBoard { boardMsg | newValue = input }
    in
    td []
        [ input
            [ type_ "text"
            , value tile.value
            , class (immutableClassStr tile)
            , onInput helper
            ]
            []
        ]


rowToTr : List ( ( Int, Int ), Tile ) -> Html Msg
rowToTr row =
    tr [] <| List.map tileToInput row


view : Model -> Html Msg
view model =
    let
        indexRows =
            getIndices

        tileRows =
            List.map (\idxRow -> rowToTr <| List.map (\tup -> ( tup, getTile tup model.board )) idxRow) indexRows
    in
    div [ class "content" ]
        [ table [ id "grid" ] <| tileRows
        , button
            [ onClick ClearBoard ]
            [ text "clear" ]
        , button
            [ onClick (GenerateBoard preSolved) ]
            [ text "new puzzle" ]
        , button
            [ onClick SolveBoard ]
            [ text "solve" ]
        ]
