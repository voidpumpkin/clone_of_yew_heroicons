use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M20%2E03%204%2E72a%2E75%2E75%200%20010%201%2E06l%2D7%2E5%207%2E5a%2E75%2E75%200%2001%2D1%2E06%200l%2D7%2E5%2D7%2E5a%2E75%2E75%200%20011%2E06%2D1%2E06L12%2011%2E69l6%2E97%2D6%2E97a%2E75%2E75%200%20011%2E06%200zm0%206a%2E75%2E75%200%20010%201%2E06l%2D7%2E5%207%2E5a%2E75%2E75%200%2001%2D1%2E06%200l%2D7%2E5%2D7%2E5a%2E75%2E75%200%20111%2E06%2D1%2E06L12%2017%2E69l6%2E97%2D6%2E97a%2E75%2E75%200%20011%2E06%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChevronDoubleDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M20.03 4.72a.75.75 0 010 1.06l-7.5 7.5a.75.75 0 01-1.06 0l-7.5-7.5a.75.75 0 011.06-1.06L12 11.69l6.97-6.97a.75.75 0 011.06 0zm0 6a.75.75 0 010 1.06l-7.5 7.5a.75.75 0 01-1.06 0l-7.5-7.5a.75.75 0 111.06-1.06L12 17.69l6.97-6.97a.75.75 0 011.06 0z" clip-rule="evenodd"/>
</svg>
  }
}
