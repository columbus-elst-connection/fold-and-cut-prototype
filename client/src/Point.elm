module Point exposing (Point, encodePoint)

import Json.Encode as Encode exposing (Value)


type alias Point a =
    ( a, a )


encodePoint: (a -> Value) -> Point a -> Value
encodePoint toValue ( x, y ) =
    Encode.list toValue [ x, y ]
