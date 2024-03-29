use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M11%2E097%201%2E515a%2E75%2E75%200%2001%2E589%2E882L10%2E666%207%2E5h4%2E47l1%2E079%2D5%2E397a%2E75%2E75%200%20111%2E47%2E294L16%2E665%207%2E5h3%2E585a%2E75%2E75%200%20010%201%2E5h%2D3%2E885l%2D1%2E2%206h3%2E585a%2E75%2E75%200%20010%201%2E5h%2D3%2E885l%2D1%2E08%205%2E397a%2E75%2E75%200%2011%2D1%2E47%2D%2E294l1%2E02%2D5%2E103h%2D4%2E47l%2D1%2E08%205%2E397a%2E75%2E75%200%2001%2D1%2E47%2D%2E294l1%2E02%2D5%2E103H3%2E75a%2E75%2E75%200%20110%2D1%2E5h3%2E885l1%2E2%2D6H5%2E25a%2E75%2E75%200%20010%2D1%2E5h3%2E885l1%2E08%2D5%2E397a%2E75%2E75%200%2001%2E882%2D%2E588zM10%2E365%209l%2D1%2E2%206h4%2E47l1%2E2%2D6h%2D4%2E47z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HashtagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M11.097 1.515a.75.75 0 01.589.882L10.666 7.5h4.47l1.079-5.397a.75.75 0 111.47.294L16.665 7.5h3.585a.75.75 0 010 1.5h-3.885l-1.2 6h3.585a.75.75 0 010 1.5h-3.885l-1.08 5.397a.75.75 0 11-1.47-.294l1.02-5.103h-4.47l-1.08 5.397a.75.75 0 01-1.47-.294l1.02-5.103H3.75a.75.75 0 110-1.5h3.885l1.2-6H5.25a.75.75 0 010-1.5h3.885l1.08-5.397a.75.75 0 01.882-.588zM10.365 9l-1.2 6h4.47l1.2-6h-4.47z" clip-rule="evenodd"/>
</svg>
  }
}
