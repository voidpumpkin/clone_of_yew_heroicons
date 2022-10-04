use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M16%2E5%206a3%203%200%2000%2D3%2D3H6a3%203%200%2000%2D3%203v7%2E5a3%203%200%20003%203v%2D6A4%2E5%204%2E5%200%200110%2E5%206h6z%22%2F%3E%20%3Cpath%20d%3D%22M18%207%2E5a3%203%200%20013%203V18a3%203%200%2001%2D3%203h%2D7%2E5a3%203%200%2001%2D3%2D3v%2D7%2E5a3%203%200%20013%2D3H18z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Square2StackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M16.5 6a3 3 0 00-3-3H6a3 3 0 00-3 3v7.5a3 3 0 003 3v-6A4.5 4.5 0 0110.5 6h6z"/>
  <path d="M18 7.5a3 3 0 013 3V18a3 3 0 01-3 3h-7.5a3 3 0 01-3-3v-7.5a3 3 0 013-3H18z"/>
</svg>
  }
}