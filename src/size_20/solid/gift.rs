use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M14%206a2%2E5%202%2E5%200%2000%2D4%2D3%202%2E5%202%2E5%200%2000%2D4%203H3%2E25C2%2E56%206%202%206%2E56%202%207%2E25v%2E5C2%208%2E44%202%2E56%209%203%2E25%209h6V6h1%2E5v3h6C17%2E44%209%2018%208%2E44%2018%207%2E75v%2D%2E5C18%206%2E56%2017%2E44%206%2016%2E75%206H14zm%2D1%2D1%2E5a1%201%200%2001%2D1%201h%2D1v%2D1a1%201%200%20112%200zm%2D6%200a1%201%200%20001%201h1v%2D1a1%201%200%2000%2D2%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M9%2E25%2010%2E5H3v4%2E75A2%2E75%202%2E75%200%20005%2E75%2018h3%2E5v%2D7%2E5zM10%2E75%2018v%2D7%2E5H17v4%2E75A2%2E75%202%2E75%200%200114%2E25%2018h%2D3%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn GiftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M14 6a2.5 2.5 0 00-4-3 2.5 2.5 0 00-4 3H3.25C2.56 6 2 6.56 2 7.25v.5C2 8.44 2.56 9 3.25 9h6V6h1.5v3h6C17.44 9 18 8.44 18 7.75v-.5C18 6.56 17.44 6 16.75 6H14zm-1-1.5a1 1 0 01-1 1h-1v-1a1 1 0 112 0zm-6 0a1 1 0 001 1h1v-1a1 1 0 00-2 0z" clip-rule="evenodd"/>
  <path d="M9.25 10.5H3v4.75A2.75 2.75 0 005.75 18h3.5v-7.5zM10.75 18v-7.5H17v4.75A2.75 2.75 0 0114.25 18h-3.5z"/>
</svg>
  }
}