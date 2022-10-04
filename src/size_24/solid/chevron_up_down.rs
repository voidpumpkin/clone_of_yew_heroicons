use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M11%2E47%204%2E72a%2E75%2E75%200%20011%2E06%200l3%2E75%203%2E75a%2E75%2E75%200%2001%2D1%2E06%201%2E06L12%206%2E31%208%2E78%209%2E53a%2E75%2E75%200%2001%2D1%2E06%2D1%2E06l3%2E75%2D3%2E75zm%2D3%2E75%209%2E75a%2E75%2E75%200%20011%2E06%200L12%2017%2E69l3%2E22%2D3%2E22a%2E75%2E75%200%20111%2E06%201%2E06l%2D3%2E75%203%2E75a%2E75%2E75%200%2001%2D1%2E06%200l%2D3%2E75%2D3%2E75a%2E75%2E75%200%20010%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChevronUpDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M11.47 4.72a.75.75 0 011.06 0l3.75 3.75a.75.75 0 01-1.06 1.06L12 6.31 8.78 9.53a.75.75 0 01-1.06-1.06l3.75-3.75zm-3.75 9.75a.75.75 0 011.06 0L12 17.69l3.22-3.22a.75.75 0 111.06 1.06l-3.75 3.75a.75.75 0 01-1.06 0l-3.75-3.75a.75.75 0 010-1.06z" clip-rule="evenodd"/>
</svg>
  }
}
