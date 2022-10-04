use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E22%203%2E22A%2E75%2E75%200%20017%2E75%203h9A2%2E25%202%2E25%200%200119%205%2E25v9%2E5A2%2E25%202%2E25%200%200116%2E75%2017h%2D9a%2E75%2E75%200%2001%2D%2E53%2D%2E22L%2E97%2010%2E53a%2E75%2E75%200%20010%2D1%2E06l6%2E25%2D6%2E25zm3%2E06%204a%2E75%2E75%200%2010%2D1%2E06%201%2E06L10%2E94%2010l%2D1%2E72%201%2E72a%2E75%2E75%200%20101%2E06%201%2E06L12%2011%2E06l1%2E72%201%2E72a%2E75%2E75%200%20101%2E06%2D1%2E06L13%2E06%2010l1%2E72%2D1%2E72a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06L12%208%2E94l%2D1%2E72%2D1%2E72z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BackspaceIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M7.22 3.22A.75.75 0 017.75 3h9A2.25 2.25 0 0119 5.25v9.5A2.25 2.25 0 0116.75 17h-9a.75.75 0 01-.53-.22L.97 10.53a.75.75 0 010-1.06l6.25-6.25zm3.06 4a.75.75 0 10-1.06 1.06L10.94 10l-1.72 1.72a.75.75 0 101.06 1.06L12 11.06l1.72 1.72a.75.75 0 101.06-1.06L13.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L12 8.94l-1.72-1.72z" clip-rule="evenodd"/>
</svg>
  }
}