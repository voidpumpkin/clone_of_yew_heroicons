use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2E5%203%2E75a6%206%200%2000%2D5%2E98%206%2E496A5%2E25%205%2E25%200%20006%2E75%2020%2E25H18a4%2E5%204%2E5%200%20002%2E206%2D8%2E423%203%2E75%203%2E75%200%2000%2D4%2E133%2D4%2E303A6%2E001%206%2E001%200%200010%2E5%203%2E75zm2%2E25%206a%2E75%2E75%200%2000%2D1%2E5%200v4%2E94l%2D1%2E72%2D1%2E72a%2E75%2E75%200%2000%2D1%2E06%201%2E06l3%203a%2E75%2E75%200%20001%2E06%200l3%2D3a%2E75%2E75%200%2010%2D1%2E06%2D1%2E06l%2D1%2E72%201%2E72V9%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CloudArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10.5 3.75a6 6 0 00-5.98 6.496A5.25 5.25 0 006.75 20.25H18a4.5 4.5 0 002.206-8.423 3.75 3.75 0 00-4.133-4.303A6.001 6.001 0 0010.5 3.75zm2.25 6a.75.75 0 00-1.5 0v4.94l-1.72-1.72a.75.75 0 00-1.06 1.06l3 3a.75.75 0 001.06 0l3-3a.75.75 0 10-1.06-1.06l-1.72 1.72V9.75z" clip-rule="evenodd"/>
</svg>
  }
}
