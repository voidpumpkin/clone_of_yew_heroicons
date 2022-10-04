use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%2E401%203%2E003c1%2E155%2D2%204%2E043%2D2%205%2E197%200l7%2E355%2012%2E748c1%2E154%202%2D%2E29%204%2E5%2D2%2E599%204%2E5H4%2E645c%2D2%2E309%200%2D3%2E752%2D2%2E5%2D2%2E598%2D4%2E5L9%2E4%203%2E003zM12%208%2E25a%2E75%2E75%200%2001%2E75%2E75v3%2E75a%2E75%2E75%200%2001%2D1%2E5%200V9a%2E75%2E75%200%2001%2E75%2D%2E75zm0%208%2E25a%2E75%2E75%200%20100%2D1%2E5%2E75%2E75%200%20000%201%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ExclamationTriangleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003zM12 8.25a.75.75 0 01.75.75v3.75a.75.75 0 01-1.5 0V9a.75.75 0 01.75-.75zm0 8.25a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd"/>
</svg>
  }
}
