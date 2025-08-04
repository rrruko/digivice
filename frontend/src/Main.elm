port module Main exposing (..)

import Browser
import Html exposing (Html, button, div, text, span, input)
import Html.Attributes exposing (checked, type_)
import Html.Events exposing (onClick, onCheck)

main = Browser.element
  { init = init
  , update = update
  , view = view
  , subscriptions = subscriptions
  }

port getModel : Int -> Cmd msg
port setCrunch : Bool -> Cmd msg

type alias Model = 
  { number : Int
  , crunch : Bool
  }

init : { initModel : Int, initCrunch: Bool } -> (Model, Cmd Msg)
init flags =
  ( { number = flags.initModel, crunch = flags.initCrunch }
  , getModel flags.initModel
  )

type Msg = Increment | Decrement | SetCrunch Bool

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
    SetCrunch b ->
      ( { model | crunch = b }
      , setCrunch b
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
    , span []  [text "crunch"]
    , input [type_ "checkbox", checked model.crunch, onCheck SetCrunch] []
    ]
