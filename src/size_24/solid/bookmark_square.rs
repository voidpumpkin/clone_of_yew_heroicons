use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%203a3%203%200%2000%2D3%203v12a3%203%200%20003%203h12a3%203%200%20003%2D3V6a3%203%200%2000%2D3%2D3H6zm1%2E5%201%2E5a%2E75%2E75%200%2000%2D%2E75%2E75V16%2E5a%2E75%2E75%200%20001%2E085%2E67L12%2015%2E089l4%2E165%202%2E083a%2E75%2E75%200%20001%2E085%2D%2E671V5%2E25a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D9z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BookmarkSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6 3a3 3 0 00-3 3v12a3 3 0 003 3h12a3 3 0 003-3V6a3 3 0 00-3-3H6zm1.5 1.5a.75.75 0 00-.75.75V16.5a.75.75 0 001.085.67L12 15.089l4.165 2.083a.75.75 0 001.085-.671V5.25a.75.75 0 00-.75-.75h-9z" clip-rule="evenodd"/>
</svg>
  }
}