use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E23%2015%2E79a%2E75%2E75%200%2001%2D%2E02%2D1%2E06l4%2E25%2D4%2E5a%2E75%2E75%200%20011%2E08%200l4%2E25%204%2E5a%2E75%2E75%200%2011%2D1%2E08%201%2E04L10%2011%2E832%206%2E29%2015%2E77a%2E75%2E75%200%2001%2D1%2E06%2E02zm0%2D6a%2E75%2E75%200%2001%2D%2E02%2D1%2E06l4%2E25%2D4%2E5a%2E75%2E75%200%20011%2E08%200l4%2E25%204%2E5a%2E75%2E75%200%2011%2D1%2E08%201%2E04L10%205%2E832%206%2E29%209%2E77a%2E75%2E75%200%2001%2D1%2E06%2E02z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChevronDoubleUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.23 15.79a.75.75 0 01-.02-1.06l4.25-4.5a.75.75 0 011.08 0l4.25 4.5a.75.75 0 11-1.08 1.04L10 11.832 6.29 15.77a.75.75 0 01-1.06.02zm0-6a.75.75 0 01-.02-1.06l4.25-4.5a.75.75 0 011.08 0l4.25 4.5a.75.75 0 11-1.08 1.04L10 5.832 6.29 9.77a.75.75 0 01-1.06.02z" clip-rule="evenodd"/>
</svg>
  }
}
