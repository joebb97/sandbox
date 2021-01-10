module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)



-- MAIN


type alias Tile =
    String


type alias Board =
    List (List Tile)


type alias Model =
    { board : Board }


main =
    Browser.sandbox { init = init, update = update, view = view }


init : Model
init =
    { board = [] }


type Msg
    = UpdateBoard


update : Msg -> Model -> Model
update msg model =
    case msg of
        UpdateBoard ->
            { board = [] }


view : Model -> Html Msg
view model =
    div [ class "content" ]
        [ table [ id "grid" ]
            [ tr []
                [ td [] [ input [ type_ "text", value "3" ] [] ]
                , td [] [ input [ type_ "text", value "3" ] [] ]
                ]
            ]
        ]
