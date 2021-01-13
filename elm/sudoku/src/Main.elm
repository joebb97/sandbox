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
    { board : Board, solved : Bool }


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
    { board = new_rows, solved = False }


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


update : Msg -> Model -> Model
update msg model =
    let
        _ =
            Debug.log "msg" <| Debug.toString msg
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
