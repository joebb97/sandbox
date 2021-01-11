module Main exposing (..)

import Array exposing (Array)
import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)



-- MAIN


type alias Tile =
    { value : Maybe Int, rowID : Int, colID : Int, possibleVals : Array Int }


type alias Board =
    Array (Array Tile)


type alias Model =
    { board : Board }


rowSize =
    9


main =
    Browser.sandbox { init = init, update = update, view = view }


init : Model
init =
    let
        rows =
            Array.repeat rowSize <|
                Array.map
                    (\val -> { colID = val, rowID = -1, value = Nothing, possibleVals = Array.empty })
                <|
                    Array.fromList (List.range 0 8)

        new_rows =
            Array.indexedMap (\idx row -> Array.map (\val -> { val | rowID = idx }) row) rows
    in
    { board = new_rows }


type alias UpdateBoardMsg =
    { rowID : Int, colID : Int, newValue : Int }


type Msg
    = UpdateBoard UpdateBoardMsg


default_tile : Tile
default_tile =
    { value = Nothing, rowID = -1, colID = -1, possibleVals = Array.empty }


update : Msg -> Model -> Model
update msg model =
    case msg of
        UpdateBoard recMsg ->
            let
                the_row =
                    Maybe.withDefault Array.empty <| Array.get recMsg.rowID model.board

                the_rec =
                    Maybe.withDefault default_tile <| Array.get recMsg.colID the_row

                output =
                    Array.set recMsg.colID { the_rec | value = Just recMsg.newValue } the_row
            in
            { board = Array.set recMsg.rowID output model.board }


tileToInput : Tile -> Html Msg
tileToInput tile =
    let
        actualVal =
            case tile.value of
                Just value ->
                    String.fromInt value

                Nothing ->
                    ""
    in
    td [] [ input [ type_ "text", value actualVal ] [] ]


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
