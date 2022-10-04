use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%202a%2E75%2E75%200%2001%2E75%2E75v12%2E59l1%2E95%2D2%2E1a%2E75%2E75%200%20111%2E1%201%2E02l%2D3%2E25%203%2E5a%2E75%2E75%200%2001%2D1%2E1%200l%2D3%2E25%2D3%2E5a%2E75%2E75%200%20111%2E1%2D1%2E02l1%2E95%202%2E1V2%2E75A%2E75%2E75%200%200110%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowLongDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 2a.75.75 0 01.75.75v12.59l1.95-2.1a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 111.1-1.02l1.95 2.1V2.75A.75.75 0 0110 2z" clip-rule="evenodd"/>
</svg>
  }
}