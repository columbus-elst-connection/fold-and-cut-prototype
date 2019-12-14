module Figure exposing (..)

-- elm install avh4/elm-color

import Browser
import Canvas exposing (Renderable, circle, lineTo, path, rect)
import Canvas.Settings as Setting
import Color
import Html exposing (Html)
import Html.Attributes exposing (style)
import Html.Events.Extra.Pointer as Pointer


main : Program () Model Message
main =
    Browser.element
        { init = \_ -> ( init, Cmd.none )
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


init : Model
init =
    empty


type alias Model =
    { width : Int
    , height : Int
    , current : Maybe (Point Float)
    , figure : List (Point Int)
    }


type alias Point a =
    ( a, a )


empty : Model
empty =
    { width = 512, height = 512, current = Nothing, figure = [] }


addPoint : ( Int, Int ) -> Model -> Model
addPoint point model =
    let
        figure =
            point :: model.figure
    in
    { model | figure = figure }


view : Model -> Html Message
view model =
    let
        width =
            toFloat model.width

        height =
            toFloat model.height
    in
    Canvas.toHtml ( model.width, model.height )
        [ style "border" "1px solid black"
        , Pointer.onDown AddPoint
        , Pointer.onEnter Subscribe
        , Pointer.onLeave Unsubscribe
        , Pointer.onMove Move
        ]
        [ Canvas.shapes [ Setting.fill Color.lightGrey ] [ rect ( 0, 0 ) width height ]
        , renderFigure model.figure
        , renderCrossHair model.current
        ]


renderFigure : List (Point Int) -> Renderable
renderFigure points =
    let
        canvasPoints =
            points
                |> List.map toCanvasPoint

        toCanvasPoint : Point Int -> Point Float
        toCanvasPoint ( x, y ) =
            ( toFloat x, toFloat y )

        start =
            canvasPoints
                |> List.head

        segments =
            canvasPoints
                |> List.tail
                |> Maybe.map (List.map lineTo)
                |> Maybe.withDefault []

        shape =
            start
                |> Maybe.map (\s -> path s segments)

        shapes =
            shape
                |> Maybe.map (\s -> [ s ])
                |> Maybe.withDefault []
    in
    Canvas.shapes [ Setting.stroke (Color.rgba 0 0 0 1) ]
        shapes


renderCrossHair : Maybe (Point Float) -> Renderable
renderCrossHair point =
    let
        radius =
            5.0

        shapes =
            point
                |> Maybe.map (\c -> [ circle c 10.0 ])
                |> Maybe.withDefault []
    in
    Canvas.shapes [ Setting.stroke <| Color.blue ]
        shapes


type Message
    = AddPoint Pointer.Event
    | Subscribe Pointer.Event
    | Unsubscribe Pointer.Event
    | Move Pointer.Event


update : Message -> Model -> ( Model, Cmd Message )
update message model =
    case message of
        AddPoint event ->
            let
                point =
                    event.pointer
                        |> .clientPos
                        |> toFigurePoint
            in
            ( model |> addPoint point, Cmd.none )

        Subscribe event ->
            let
                point =
                    event.pointer
                        |> .clientPos
            in
            ( { model | current = Just point }, Cmd.none )

        Unsubscribe _ ->
            ( { model | current = Nothing }, Cmd.none )

        Move event ->
            let
                point =
                    event.pointer
                        |> .clientPos

                current =
                    model.current
                        |> Maybe.map (\_ -> point)
            in
            ( { model | current = current }, Cmd.none )


toFigurePoint : Point Float -> Point Int
toFigurePoint ( x, y ) =
    ( round x, round y )


subscriptions : Model -> Sub Message
subscriptions model =
    Sub.none
