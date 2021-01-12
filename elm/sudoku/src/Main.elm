module Main exposing (..)

import Array exposing (Array)
import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)



-- MAIN


type alias Tile =
    { value : String, rowID : Int, colID : Int, possibleVals : Array Int }


type alias Board =
    Array (Array Tile)


type alias Model =
    { board : Board }


type alias UpdateBoardMsg =
    { rowID : Int, colID : Int, newValue : String }


rowSize =
    9


main =
    Browser.sandbox { init = init, update = update, view = view }


init : Model
init =
    let
        initVals =
            Array.fromList (List.range 1 9)

        rows =
            Array.repeat rowSize <|
                Array.map
                    (\val -> { colID = val, rowID = -1, value = "", possibleVals = initVals })
                <|
                    Array.fromList (List.range 0 8)

        new_rows =
            Array.indexedMap (\idx row -> Array.map (\val -> { val | rowID = idx }) row) rows
    in
    { board = new_rows }


type Msg
    = UpdateBoard UpdateBoardMsg


default_tile : Tile
default_tile =
    { value = "", rowID = -1, colID = -1, possibleVals = Array.empty }


applyUpdate : UpdateBoardMsg -> Board -> Board
applyUpdate recMsg board =
    let
        the_row =
            Maybe.withDefault Array.empty <| Array.get recMsg.rowID board

        the_rec =
            Maybe.withDefault default_tile <| Array.get recMsg.colID the_row

        output =
            Array.set recMsg.colID { the_rec | value = recMsg.newValue } the_row
    in
    Array.set recMsg.rowID output board


update: Msg -> Model -> Model
update msg model =
    let
        _ = Debug.log "msg" <| Debug.toString msg
    in
    case msg of
        UpdateBoard recMsg ->
            case String.toInt recMsg.newValue of
                Just value ->
                    if value >= 1 && value <= 9 then
                        { board = applyUpdate recMsg model.board }
                    else
                        {board = model.board}

                Nothing -> 
                    { board = applyUpdate recMsg model.board }



tileToInput : Tile -> Html Msg
tileToInput tile =
    let
        boardMsg =
            { rowID = tile.rowID, colID = tile.colID, newValue = tile.value }
        helper input = UpdateBoard { boardMsg | newValue = input}
    in
    td [] [ input [ type_ "text", value tile.value, onInput helper ] [] ]


rowToTr : Array Tile -> Html Msg
rowToTr row =
    tr [] <| Array.toList <| Array.map tileToInput row


view : Model -> Html Msg
view model =
    div [ class "content" ]
        [ table [ id "grid" ] <|
            Array.toList <|
                Array.map rowToTr model.board
        ]



-- (List.repeat 9 <|
--     tr [] <|
--         List.repeat 9 <|
--             td [] [ input [ type_ "text", value "4" ] [] ]
-- )
