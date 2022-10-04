use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E08%205%2E227A3%203%200%20016%2E979%203H17%2E02a3%203%200%20012%2E9%202%2E227l2%2E113%207%2E926A5%2E228%205%2E228%200%200018%2E75%2012H5%2E25a5%2E228%205%2E228%200%2000%2D3%2E284%201%2E153L4%2E08%205%2E227z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E25%2013%2E5a3%2E75%203%2E75%200%20100%207%2E5h13%2E5a3%2E75%203%2E75%200%20100%2D7%2E5H5%2E25zm10%2E5%204%2E5a%2E75%2E75%200%20100%2D1%2E5%2E75%2E75%200%20000%201%2E5zm3%2E75%2D%2E75a%2E75%2E75%200%2011%2D1%2E5%200%20%2E75%2E75%200%20011%2E5%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ServerIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M4.08 5.227A3 3 0 016.979 3H17.02a3 3 0 012.9 2.227l2.113 7.926A5.228 5.228 0 0018.75 12H5.25a5.228 5.228 0 00-3.284 1.153L4.08 5.227z"/>
  <path fill-rule="evenodd" d="M5.25 13.5a3.75 3.75 0 100 7.5h13.5a3.75 3.75 0 100-7.5H5.25zm10.5 4.5a.75.75 0 100-1.5.75.75 0 000 1.5zm3.75-.75a.75.75 0 11-1.5 0 .75.75 0 011.5 0z" clip-rule="evenodd"/>
</svg>
  }
}