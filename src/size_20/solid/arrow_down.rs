use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%203a%2E75%2E75%200%2001%2E75%2E75v10%2E638l3%2E96%2D4%2E158a%2E75%2E75%200%20111%2E08%201%2E04l%2D5%2E25%205%2E5a%2E75%2E75%200%2001%2D1%2E08%200l%2D5%2E25%2D5%2E5a%2E75%2E75%200%20111%2E08%2D1%2E04l3%2E96%204%2E158V3%2E75A%2E75%2E75%200%200110%203z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 3a.75.75 0 01.75.75v10.638l3.96-4.158a.75.75 0 111.08 1.04l-5.25 5.5a.75.75 0 01-1.08 0l-5.25-5.5a.75.75 0 111.08-1.04l3.96 4.158V3.75A.75.75 0 0110 3z" clip-rule="evenodd"/>
</svg>
  }
}