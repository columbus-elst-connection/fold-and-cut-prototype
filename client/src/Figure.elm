module Figure exposing (..)

import Browser
import Canvas exposing (Renderable, Shape, circle, lineTo, path, rect)
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
    , currentPoint : Maybe (Point Float)
    , currentFigure : List (Point Int)
    , figure : List Figure
    }


type Figure
    = Open (List (Point Int))
    | Closed (List (Point Int))


type alias Point a =
    ( a, a )


empty : Model
empty =
    { width = 512, height = 512, currentPoint = Nothing, currentFigure = [], figure = [] }


addPoint : ( Int, Int ) -> Model -> Model
addPoint point model =
    let
        currentFigure =
            point :: model.currentFigure
    in
    { model | currentFigure = currentFigure }


addFigure : Figure -> Model -> Model
addFigure figure model =
    { model | figure = figure :: model.figure }


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
        , renderFigure <| [ Open model.currentFigure ]
        , renderCrossHair model.currentPoint
        ]


renderFigure : List Figure -> Renderable
renderFigure figures =
    let
        shapes =
            renderShapes figures
    in
    Canvas.shapes [ Setting.stroke (Color.rgba 0 0 0 1) ]
        shapes


renderShapes : List Figure -> List Shape
renderShapes =
    List.concatMap renderShape


renderShape : Figure -> List Shape
renderShape figure =
    case figure of
        Open points ->
            points
                |> renderPoints
                |> Maybe.map (\s -> [ s ])
                |> Maybe.withDefault []

        Closed points ->
            points
                |> close
                |> renderPoints
                |> Maybe.map (\s -> [ s ])
                |> Maybe.withDefault []


close : List a -> List a
close z =
    case z of
        [] ->
            []

        p :: _ ->
            z ++ [ p ]


renderPoints : List (Point Int) -> Maybe Shape
renderPoints points =
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
    in
    start
        |> Maybe.map (\s -> path s segments)


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
                    event
                        |> .pointer
                        |> .clientPos
                        |> toFigurePoint

                keys =
                    event
                        |> .pointer
                        |> .keys
            in
            case ( keys.shift, keys.ctrl ) of
                ( True, _ ) ->
                    let
                        figure =
                            Open <| point :: model.currentFigure

                        nextModel =
                            { model | currentFigure = [] }
                                |> addFigure figure
                    in
                    ( nextModel, Cmd.none )

                ( _, True ) ->
                    let
                        figure =
                            Closed <| point :: model.currentFigure

                        nextModel =
                            { model | currentFigure = [] }
                                |> addFigure figure
                    in
                    ( nextModel, Cmd.none )

                _ ->
                    ( model |> addPoint point, Cmd.none )

        Subscribe event ->
            let
                point =
                    event.pointer
                        |> .clientPos
            in
            ( { model | currentPoint = Just point }, Cmd.none )

        Unsubscribe _ ->
            ( { model | currentPoint = Nothing }, Cmd.none )

        Move event ->
            let
                point =
                    event.pointer
                        |> .clientPos

                currentPoint =
                    model.currentPoint
                        |> Maybe.map (\_ -> point)
            in
            ( { model | currentPoint = currentPoint }, Cmd.none )


toFigurePoint : Point Float -> Point Int
toFigurePoint ( x, y ) =
    ( round x, round y )


subscriptions : Model -> Sub Message
subscriptions model =
    Sub.none
