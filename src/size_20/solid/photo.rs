use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%205%2E25A2%2E25%202%2E25%200%20013%2E25%203h13%2E5A2%2E25%202%2E25%200%200119%205%2E25v9%2E5A2%2E25%202%2E25%200%200116%2E75%2017H3%2E25A2%2E25%202%2E25%200%20011%2014%2E75v%2D9%2E5zm1%2E5%205%2E81v3%2E69c0%20%2E414%2E336%2E75%2E75%2E75h13%2E5a%2E75%2E75%200%2000%2E75%2D%2E75v%2D2%2E69l%2D2%2E22%2D2%2E219a%2E75%2E75%200%2000%2D1%2E06%200l%2D1%2E91%201%2E909%2E47%2E47a%2E75%2E75%200%2011%2D1%2E06%201%2E06L6%2E53%208%2E091a%2E75%2E75%200%2000%2D1%2E06%200l%2D2%2E97%202%2E97zM12%207a1%201%200%2011%2D2%200%201%201%200%20012%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PhotoIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1 5.25A2.25 2.25 0 013.25 3h13.5A2.25 2.25 0 0119 5.25v9.5A2.25 2.25 0 0116.75 17H3.25A2.25 2.25 0 011 14.75v-9.5zm1.5 5.81v3.69c0 .414.336.75.75.75h13.5a.75.75 0 00.75-.75v-2.69l-2.22-2.219a.75.75 0 00-1.06 0l-1.91 1.909.47.47a.75.75 0 11-1.06 1.06L6.53 8.091a.75.75 0 00-1.06 0l-2.97 2.97zM12 7a1 1 0 11-2 0 1 1 0 012 0z" clip-rule="evenodd"/>
</svg>
  }
}