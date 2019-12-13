module Figure exposing (..)

-- elm install avh4/elm-color

import Browser
import Canvas exposing (..)
import Canvas.Settings exposing (..)
import Color
import Html exposing (Html)
import Html.Attributes exposing (style)


main : Program () () msg
main =
    Browser.element
        { init = \_ -> ( init, Cmd.none )
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

init =
    ()

view : model -> Html msg
view _ =
    let
        width =
            500

        height =
            300
    in
    Canvas.toHtml ( width, height )
        [ style "border" "1px solid black" ]
        [ shapes [ fill Color.white ] [ rect ( 0, 0 ) width height ]
        , renderSquare
        ]


renderSquare =
    shapes [ fill (Color.rgba 0 0 0 1) ]
        [ rect ( 0, 0 ) 100 50 ]


update : msg -> model -> ( model, Cmd msg )
update _ m =
    (m, Cmd.none)

subscriptions : model -> Sub msg
subscriptions model =
    Sub.none
