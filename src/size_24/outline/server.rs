use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2E75%2017%2E25v%2D%2E228a4%2E5%204%2E5%200%2000%2D%2E12%2D1%2E03l%2D2%2E268%2D9%2E64a3%2E375%203%2E375%200%2000%2D3%2E285%2D2%2E602H7%2E923a3%2E375%203%2E375%200%2000%2D3%2E285%202%2E602l%2D2%2E268%209%2E64a4%2E5%204%2E5%200%2000%2D%2E12%201%2E03v%2E228m19%2E5%200a3%203%200%2001%2D3%203H5%2E25a3%203%200%2001%2D3%2D3m19%2E5%200a3%203%200%2000%2D3%2D3H5%2E25a3%203%200%2000%2D3%203m16%2E5%200h%2E008v%2E008h%2D%2E008v%2D%2E008zm%2D3%200h%2E008v%2E008h%2D%2E008v%2D%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ServerIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 17.25v-.228a4.5 4.5 0 00-.12-1.03l-2.268-9.64a3.375 3.375 0 00-3.285-2.602H7.923a3.375 3.375 0 00-3.285 2.602l-2.268 9.64a4.5 4.5 0 00-.12 1.03v.228m19.5 0a3 3 0 01-3 3H5.25a3 3 0 01-3-3m19.5 0a3 3 0 00-3-3H5.25a3 3 0 00-3 3m16.5 0h.008v.008h-.008v-.008zm-3 0h.008v.008h-.008v-.008z"/>
</svg>
  }
}