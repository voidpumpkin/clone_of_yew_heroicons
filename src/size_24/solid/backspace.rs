use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E515%2010%2E674a1%2E875%201%2E875%200%20000%202%2E652L8%2E89%2019%2E7c%2E352%2E351%2E829%2E549%201%2E326%2E549H19%2E5a3%203%200%20003%2D3V6%2E75a3%203%200%2000%2D3%2D3h%2D9%2E284c%2D%2E497%200%2D%2E974%2E198%2D1%2E326%2E55l%2D6%2E375%206%2E374zM12%2E53%209%2E22a%2E75%2E75%200%2010%2D1%2E06%201%2E06L13%2E19%2012l%2D1%2E72%201%2E72a%2E75%2E75%200%20101%2E06%201%2E06l1%2E72%2D1%2E72%201%2E72%201%2E72a%2E75%2E75%200%20101%2E06%2D1%2E06L15%2E31%2012l1%2E72%2D1%2E72a%2E75%2E75%200%2010%2D1%2E06%2D1%2E06l%2D1%2E72%201%2E72%2D1%2E72%2D1%2E72z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BackspaceIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.515 10.674a1.875 1.875 0 000 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 003-3V6.75a3 3 0 00-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374zM12.53 9.22a.75.75 0 10-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 101.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 101.06-1.06L15.31 12l1.72-1.72a.75.75 0 10-1.06-1.06l-1.72 1.72-1.72-1.72z" clip-rule="evenodd"/>
</svg>
  }
}