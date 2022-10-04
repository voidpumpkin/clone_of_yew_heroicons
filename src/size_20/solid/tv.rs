use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%205h12v7H4V5z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%203%2E5A1%2E5%201%2E5%200%20012%2E5%202h15A1%2E5%201%2E5%200%200119%203%2E5v10a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5H12v1%2E5h3%2E25a%2E75%2E75%200%20010%201%2E5H4%2E75a%2E75%2E75%200%20010%2D1%2E5H8V15H2%2E5A1%2E5%201%2E5%200%20011%2013%2E5v%2D10zm16%2E5%200h%2D15v10h15v%2D10z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TvIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M4 5h12v7H4V5z"/>
  <path fill-rule="evenodd" d="M1 3.5A1.5 1.5 0 012.5 2h15A1.5 1.5 0 0119 3.5v10a1.5 1.5 0 01-1.5 1.5H12v1.5h3.25a.75.75 0 010 1.5H4.75a.75.75 0 010-1.5H8V15H2.5A1.5 1.5 0 011 13.5v-10zm16.5 0h-15v10h15v-10z" clip-rule="evenodd"/>
</svg>
  }
}