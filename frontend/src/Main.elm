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

port getModel : Int -> Cmd msg

type alias Model = 
  { number : Int
  }

init : { initModel : Int } -> (Model, Cmd Msg)
init flags =
  ( { number = flags.initModel }
  , getModel flags.initModel
  )

type Msg = Increment | Decrement

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Increment ->
      ( { model | number = model.number + 1 }
      , getModel (model.number + 1)
      )
    Decrement ->
      ( { model | number = model.number - 1 }
      , getModel (model.number - 1)
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
