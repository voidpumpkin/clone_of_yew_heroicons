use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%206a3%203%200%20013%2D3h13%2E5a3%203%200%20013%203v12a3%203%200%2001%2D3%203H5%2E25a3%203%200%2001%2D3%2D3V6zm3%2E97%2E97a%2E75%2E75%200%20011%2E06%200l2%2E25%202%2E25a%2E75%2E75%200%20010%201%2E06l%2D2%2E25%202%2E25a%2E75%2E75%200%2001%2D1%2E06%2D1%2E06l1%2E72%2D1%2E72%2D1%2E72%2D1%2E72a%2E75%2E75%200%20010%2D1%2E06zm4%2E28%204%2E28a%2E75%2E75%200%20000%201%2E5h3a%2E75%2E75%200%20000%2D1%2E5h%2D3z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CommandLineIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 6a3 3 0 013-3h13.5a3 3 0 013 3v12a3 3 0 01-3 3H5.25a3 3 0 01-3-3V6zm3.97.97a.75.75 0 011.06 0l2.25 2.25a.75.75 0 010 1.06l-2.25 2.25a.75.75 0 01-1.06-1.06l1.72-1.72-1.72-1.72a.75.75 0 010-1.06zm4.28 4.28a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z" clip-rule="evenodd"/>
</svg>
  }
}