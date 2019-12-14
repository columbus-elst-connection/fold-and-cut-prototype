module Figure exposing (..)

-- elm install avh4/elm-color

import Browser
import Canvas exposing (Renderable, lineTo, path, rect)
import Canvas.Settings as Setting
import Color
import Html exposing (Html)
import Html.Attributes exposing (style)


main : Program () Model msg
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
        |> addPoint ( 0, 0 )
        |> addPoint ( 100, 100 )
        |> addPoint ( 200, 50 )


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


view : Model -> Html msg
view model =
    let
        width =
            toFloat model.width

        height =
            toFloat model.height
    in
    Canvas.toHtml ( model.width, model.height )
        [ style "border" "1px solid black" ]
        [ Canvas.shapes [ Setting.fill Color.lightGrey] [ rect ( 0, 0 ) width height ]
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


update : msg -> Model -> ( Model, Cmd msg )
update _ m =
    ( m, Cmd.none )


subscriptions : Model -> Sub msg
subscriptions model =
    Sub.none
