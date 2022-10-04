use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M6%2E28%205%2E22a%2E75%2E75%200%2000%2D1%2E06%201%2E06L8%2E94%2010l%2D3%2E72%203%2E72a%2E75%2E75%200%20101%2E06%201%2E06L10%2011%2E06l3%2E72%203%2E72a%2E75%2E75%200%20101%2E06%2D1%2E06L11%2E06%2010l3%2E72%2D3%2E72a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06L10%208%2E94%206%2E28%205%2E22z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn XMarkIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"/>
</svg>
  }
}