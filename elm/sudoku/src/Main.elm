module Main exposing (..)

import Board exposing (..)
import Browser
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)



-- MAIN


type alias Model =
    { board : Board, solved : Bool }


main =
    Browser.sandbox { init = init, update = update, view = view }


init : Model
init =
    let
        tuples =
            getIndicesCat

        initVals =
            List.range 1 9

        tup_to_rec =
            \tup ->
                { rowID = Tuple.first tup
                , colID = Tuple.second tup
                , value = ""
                , possibleVals = initVals
                }

        init_board =
            Dict.fromList <| List.map (\tup -> ( tup, tup_to_rec tup )) tuples
    in
    { board = init_board, solved = False }


type Msg
    = UpdateBoard UpdateBoardMsg


update : Msg -> Model -> Model
update msg model =
    let
        _ = Debug.log "msg" <| Debug.toString msg
    in
    case msg of
        UpdateBoard recMsg ->
            let
                cleared =
                    { recMsg | newValue = "" }
            in
            case String.toInt recMsg.newValue of
                Just value ->
                    if value >= 1 && value <= 9 then
                        { board = fixPossibleVals recMsg <| applyUpdate recMsg model.board
                        , solved = False
                        }

                    else
                        { board = model.board
                        , solved = False
                        }

                Nothing ->
                    { board = applyUpdate cleared model.board
                    , solved = False
                    }


tileToInput : Tile -> Html Msg
tileToInput tile =
    let
        boardMsg =
            { rowID = tile.rowID, colID = tile.colID, newValue = tile.value }

        helper input =
            UpdateBoard { boardMsg | newValue = input }
    in
    td [] [ input [ type_ "text", value tile.value, onInput helper ] [] ]


rowToTr : List Tile -> Html Msg
rowToTr row =
    tr [] <| List.map tileToInput row


view : Model -> Html Msg
view model =
    let
        indexRows =
            getIndices

        tileRows =
            List.map (\idxRow -> rowToTr <| List.map (\tup -> getTile tup model.board) idxRow) indexRows
    in
    div [ class "content" ]
        [ table [ id "grid" ] <| tileRows ]
