module Figure exposing (..)


import Json.Encode as Encode exposing (Value)
import Point exposing (Point, encodePoint)


type Figure
    = Open (List (Point Float))
    | Closed (List (Point Float))
    | Composed (List Figure)


encodeFigure : Figure -> Value
encodeFigure figure =
    case figure of
        Open points ->
            Encode.object
                [ ( "type", Encode.string "open" )
                , ( "points", Encode.list (encodePoint Encode.float) points )
                ]

        Closed points ->
            Encode.object
                [ ( "type", Encode.string "closed" )
                , ( "points", Encode.list (encodePoint Encode.float) points )
                ]

        Composed figures ->
            Encode.object
                [ ( "type", Encode.string "composed" )
                , ( "points", Encode.list encodeFigure figures)
                ]
