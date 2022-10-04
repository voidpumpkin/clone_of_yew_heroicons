use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%2013%2E5a8%2E25%208%2E25%200%20018%2E25%2D8%2E25%2E75%2E75%200%2001%2E75%2E75v6%2E75H18a%2E75%2E75%200%2001%2E75%2E75%208%2E25%208%2E25%200%2001%2D16%2E5%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%2E75%203a%2E75%2E75%200%2001%2E75%2D%2E75%208%2E25%208%2E25%200%20018%2E25%208%2E25%2E75%2E75%200%2001%2D%2E75%2E75h%2D7%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75V3z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChartPieIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 13.5a8.25 8.25 0 018.25-8.25.75.75 0 01.75.75v6.75H18a.75.75 0 01.75.75 8.25 8.25 0 01-16.5 0z" clip-rule="evenodd"/>
  <path fill-rule="evenodd" d="M12.75 3a.75.75 0 01.75-.75 8.25 8.25 0 018.25 8.25.75.75 0 01-.75.75h-7.5a.75.75 0 01-.75-.75V3z" clip-rule="evenodd"/>
</svg>
  }
}
