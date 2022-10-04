use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%206a3%203%200%20013%2D3h2%2E25a3%203%200%20013%203v2%2E25a3%203%200%2001%2D3%203H6a3%203%200%2001%2D3%2D3V6zm9%2E75%200a3%203%200%20013%2D3H18a3%203%200%20013%203v2%2E25a3%203%200%2001%2D3%203h%2D2%2E25a3%203%200%2001%2D3%2D3V6zM3%2015%2E75a3%203%200%20013%2D3h2%2E25a3%203%200%20013%203V18a3%203%200%2001%2D3%203H6a3%203%200%2001%2D3%2D3v%2D2%2E25zm9%2E75%200a3%203%200%20013%2D3H18a3%203%200%20013%203V18a3%203%200%2001%2D3%203h%2D2%2E25a3%203%200%2001%2D3%2D3v%2D2%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Squares2X2Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 6a3 3 0 013-3h2.25a3 3 0 013 3v2.25a3 3 0 01-3 3H6a3 3 0 01-3-3V6zm9.75 0a3 3 0 013-3H18a3 3 0 013 3v2.25a3 3 0 01-3 3h-2.25a3 3 0 01-3-3V6zM3 15.75a3 3 0 013-3h2.25a3 3 0 013 3V18a3 3 0 01-3 3H6a3 3 0 01-3-3v-2.25zm9.75 0a3 3 0 013-3H18a3 3 0 013 3V18a3 3 0 01-3 3h-2.25a3 3 0 01-3-3v-2.25z" clip-rule="evenodd"/>
</svg>
  }
}