use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M8%207a5%205%200%20113%2E61%204%2E804l%2D1%2E903%201%2E903A1%201%200%20019%2014H8v1a1%201%200%2001%2D1%201H6v1a1%201%200%2001%2D1%201H3a1%201%200%2001%2D1%2D1v%2D2a1%201%200%2001%2E293%2D%2E707L8%2E196%208%2E39A5%2E002%205%2E002%200%20018%207zm5%2D3a%2E75%2E75%200%20000%201%2E5A1%2E5%201%2E5%200%200114%2E5%207%20%2E75%2E75%200%200016%207a3%203%200%2000%2D3%2D3z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn KeyIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M8 7a5 5 0 113.61 4.804l-1.903 1.903A1 1 0 019 14H8v1a1 1 0 01-1 1H6v1a1 1 0 01-1 1H3a1 1 0 01-1-1v-2a1 1 0 01.293-.707L8.196 8.39A5.002 5.002 0 018 7zm5-3a.75.75 0 000 1.5A1.5 1.5 0 0114.5 7 .75.75 0 0016 7a3 3 0 00-3-3z" clip-rule="evenodd"/>
</svg>
  }
}