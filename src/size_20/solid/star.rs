use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2E868%202%2E884c%2D%2E321%2D%2E772%2D1%2E415%2D%2E772%2D1%2E736%200l%2D1%2E83%204%2E401%2D4%2E753%2E381c%2D%2E833%2E067%2D1%2E171%201%2E107%2D%2E536%201%2E651l3%2E62%203%2E102%2D1%2E106%204%2E637c%2D%2E194%2E813%2E691%201%2E456%201%2E405%201%2E02L10%2015%2E591l4%2E069%202%2E485c%2E713%2E436%201%2E598%2D%2E207%201%2E404%2D1%2E02l%2D1%2E106%2D4%2E637%203%2E62%2D3%2E102c%2E635%2D%2E544%2E297%2D1%2E584%2D%2E536%2D1%2E65l%2D4%2E752%2D%2E382%2D1%2E831%2D4%2E401z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn StarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401z" clip-rule="evenodd"/>
</svg>
  }
}