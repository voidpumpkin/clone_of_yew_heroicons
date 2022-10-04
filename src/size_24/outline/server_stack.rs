use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M5%2E25%2014%2E25h13%2E5m%2D13%2E5%200a3%203%200%2001%2D3%2D3m3%203a3%203%200%20100%206h13%2E5a3%203%200%20100%2D6m%2D16%2E5%2D3a3%203%200%20013%2D3h13%2E5a3%203%200%20013%203m%2D19%2E5%200a4%2E5%204%2E5%200%2001%2E9%2D2%2E7L5%2E737%205%2E1a3%2E375%203%2E375%200%20012%2E7%2D1%2E35h7%2E126c1%2E062%200%202%2E062%2E5%202%2E7%201%2E35l2%2E587%203%2E45a4%2E5%204%2E5%200%2001%2E9%202%2E7m0%200a3%203%200%2001%2D3%203m0%203h%2E008v%2E008h%2D%2E008v%2D%2E008zm0%2D6h%2E008v%2E008h%2D%2E008v%2D%2E008zm%2D3%206h%2E008v%2E008h%2D%2E008v%2D%2E008zm0%2D6h%2E008v%2E008h%2D%2E008v%2D%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ServerStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 14.25h13.5m-13.5 0a3 3 0 01-3-3m3 3a3 3 0 100 6h13.5a3 3 0 100-6m-16.5-3a3 3 0 013-3h13.5a3 3 0 013 3m-19.5 0a4.5 4.5 0 01.9-2.7L5.737 5.1a3.375 3.375 0 012.7-1.35h7.126c1.062 0 2.062.5 2.7 1.35l2.587 3.45a4.5 4.5 0 01.9 2.7m0 0a3 3 0 01-3 3m0 3h.008v.008h-.008v-.008zm0-6h.008v.008h-.008v-.008zm-3 6h.008v.008h-.008v-.008zm0-6h.008v.008h-.008v-.008z"/>
</svg>
  }
}