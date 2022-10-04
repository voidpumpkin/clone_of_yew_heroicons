use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E5%206a4%2E5%204%2E5%200%20119%200%204%2E5%204%2E5%200%2001%2D9%200zM3%2E751%2020%2E105a8%2E25%208%2E25%200%200116%2E498%200%20%2E75%2E75%200%2001%2D%2E437%2E695A18%2E683%2018%2E683%200%200112%2022%2E5c%2D2%2E786%200%2D5%2E433%2D%2E608%2D7%2E812%2D1%2E7a%2E75%2E75%200%2001%2D%2E437%2D%2E695z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M7.5 6a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM3.751 20.105a8.25 8.25 0 0116.498 0 .75.75 0 01-.437.695A18.683 18.683 0 0112 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 01-.437-.695z" clip-rule="evenodd"/>
</svg>
  }
}
