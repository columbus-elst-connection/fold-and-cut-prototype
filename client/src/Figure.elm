module Figure exposing (..)

-- elm install avh4/elm-color

import Browser
import Canvas exposing (Renderable, lineTo, path, rect)
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
    , figure : List (Point Int)
    }


type alias Point a =
    ( a, a )


empty : Model
empty =
    { width = 512, height = 512, figure = [] }


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
        ]
        [ Canvas.shapes [ Setting.fill Color.lightGrey ] [ rect ( 0, 0 ) width height ]
        , renderFigure model.figure
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


type Message
    = AddPoint Pointer.Event


update : Message -> Model -> ( Model, Cmd Message )
update message model =
    case message of
        AddPoint event ->
            let
                point =
                    event.pointer
                        |> .clientPos
                        |> toFigurePoint

                toFigurePoint : Point Float -> Point Int
                toFigurePoint ( x, y ) =
                    ( round x, round y )
            in
            ( model |> addPoint point, Cmd.none )


subscriptions : Model -> Sub Message
subscriptions model =
    Sub.none
