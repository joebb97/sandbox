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
                -- _ =
                --     Debug.log "Attempting to solve" "thinking ..."

                ( newBoard, solved ) =
                    solveBoard model.board

                -- _ =
                --     if solved == Halt then
                --         Debug.log "Couldn't solve" "Darn"

                --     else
                --         Debug.log "Success!" "yay"
            in
            ( { model | board = newBoard, solved = solved == Success }, Cmd.none )

        DefaultBoard ->
            ( { model | board = wikipediaBoard, solved = False}, Cmd.none)


type SearchState
    = Halt
    | Continue
    | Success


tryValue : ( Int, Int, Int ) -> ( Board, SearchState ) -> ( Board, SearchState )
tryValue triple boardSolvedPair =
    let
        ( board, _ ) =
            boardSolvedPair

        ( possibleVal, row, col ) =
            triple

        tile =
            getTile ( row, col ) board

        updateMsg =
            { newValue = String.fromInt possibleVal
            , oldValue = tile.value
            , rowID = row
            , colID = col
            , newImmutable = False
            }

        ( newBoard, newState ) =
            solveBoard <| applyUpdateAndFix updateMsg board
    in
    ( newBoard, newState )


solveTile : ( Int, Int ) -> Tile -> ( Board, SearchState ) -> ( Board, SearchState )
solveTile key tile boardSolvedPair =
    let
        ( row, col ) =
            key

        ( board, searchState ) =
            boardSolvedPair

        ( newBoard, newSolved ) =
            if searchState == Halt then
                ( board, Halt )

            else if searchState == Success then
                ( board, Success )

            else if Set.isEmpty tile.possibleVals then
                if not (validValue tile.value) then
                    ( board, Halt )

                else
                    ( board, Continue )

            else
                let
                    asList =
                        Set.toList tile.possibleVals

                    outcomes =
                        List.map
                            (\possibleVal -> tryValue ( possibleVal, row, col ) boardSolvedPair)
                            asList

                    stillHope =
                        List.filter (\outcome -> Tuple.second outcome == Continue) outcomes

                    succeeded =
                        List.filter (\outcome -> Tuple.second outcome == Success) outcomes

                    failed = 
                        List.filter (\outcome -> Tuple.second outcome == Halt) outcomes
                in
                if not (List.isEmpty succeeded) then
                    Maybe.withDefault ( board, Halt ) <| List.head succeeded
                else if not (List.isEmpty stillHope) then
                    Maybe.withDefault ( board, Halt ) <| List.head stillHope
                else
                    Maybe.withDefault ( board, Halt) <| List.head failed
    in
    ( newBoard, newSolved )


isSolved : Board -> Bool
isSolved board =
    let
        helper tup solved =
            let
                tile =
                    getTile tup board
            in
            if validValue tile.value then
                True && solved

            else
                False
    in
    List.foldl helper True getIndicesCat


solveBoard : Board -> ( Board, SearchState )
solveBoard board =
    let
        ( newBoard, solved ) =
            if isSolved board then
                ( board, Success )

            else
                Dict.foldl solveTile ( board, Continue ) board
    in
    ( newBoard, solved )


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


solvedText: Bool -> String
solvedText solved = 
    if solved then
        "The puzzle is solved, hooray!"
    else
        "The puzzle hasn't been solved or has no solution" 

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
        , button
            [ onClick DefaultBoard ]
            [ text "wikipedia board" ]
        , br [] []
        , p [] [ text (solvedText model.solved) ]
        ]
        
