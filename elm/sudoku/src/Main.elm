module Main exposing (..)

import Board exposing (..)
import BoardGen exposing (..)
import BoardSolve exposing (..)
import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Process
import Random
import Set exposing (Set)
import Task exposing (Task)
import Time
import Util exposing (..)



-- MAIN


type alias Model =
    { board : Board
    , solvedText : String
    }


main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = \_ -> Sub.none
        }


defaultSolvedText : String
defaultSolvedText =
    "The puzzle hasn't been solved"


getSolvedText : Bool -> String
getSolvedText solved =
    if solved then
        "The puzzle is solved, hooray!"

    else
        "The puzzle has no solution"


initialModel : Model
initialModel =
    { board = defaultBoard, solvedText = defaultSolvedText }


init : () -> ( Model, Cmd Msg )
init _ =
    ( initialModel, Cmd.none )


type Msg
    = UpdateBoard UpdateBoardMsg
    | ClearBoard
    | GenerateBoard Int
    | NewRandom Board
    | StartSolveBoard
    | DoneSolveBoard ( Board, SearchState )
    | DefaultBoard


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

                -- _ =
                --     Debug.log "UpdateBoard Tile" <| Debug.toString tile
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
                ( { board = newBoard, solvedText = defaultSolvedText }, Cmd.none )

            else
                ( model, Cmd.none )

        ClearBoard ->
            ( initialModel, Cmd.none )

        GenerateBoard _ ->
            ( { model | solvedText = defaultSolvedText }
            , Random.generate
                NewRandom
                (preFilledGen |> Random.andThen (boardFromPositions defaultBoard))
            )

        NewRandom newRand ->
            ( { model | board = newRand }, Cmd.none )

        StartSolveBoard ->
            ( { model | solvedText = "Attempting to solve, thinking ..." }, runSearchTask model )

        DoneSolveBoard ( newBoard, solved ) ->
            let
                newSolvedText =
                    getSolvedText <| solved == Success
            in
            ( { model | board = newBoard, solvedText = newSolvedText }, Cmd.none )

        DefaultBoard ->
            ( { model | board = wikipediaBoard, solvedText = defaultSolvedText }, Cmd.none )


runSearchTask : Model -> Cmd Msg
runSearchTask model =
    -- 20 milliseconds
    Process.sleep 20
        |> Task.perform (\_ -> DoneSolveBoard (solveBoard model.board))


immutableClassStr : Tile -> String
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
            [ onClick StartSolveBoard ]
            [ text "solve" ]
        , button
            [ onClick DefaultBoard ]
            [ text "wikipedia board" ]
        , br [] []
        , p [] [ text model.solvedText ]
        ]
