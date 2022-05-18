port module Main exposing (..)

import Browser
import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)

main = Browser.element
  { init = init
  , update = update
  , view = view
  , subscriptions = subscriptions
  }

port getModel : String -> Cmd msg

type alias Model = 
  { number : Int
  }

init : () -> (Model, Cmd Msg)
init flags =
  ( { number = 0 }
  , Cmd.none
  )

type Msg = Increment | Decrement

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Increment ->
      ( { model | number = model.number + 1 }
      , getModel "inc"
      )
    Decrement ->
      ( { model | number = model.number - 1 }
      , getModel "dec"
      )

subscriptions : Model -> Sub Msg
subscriptions _ = Sub.none

view : Model -> Html Msg
view model =
  div
    []
    [ button [onClick Decrement] [text "-"]
    , div [] [text (String.fromInt model.number)]
    , button [onClick Increment] [text "+"]
    ]
