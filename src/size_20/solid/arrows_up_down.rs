use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E24%206%2E8a%2E75%2E75%200%20001%2E06%2D%2E04l1%2E95%2D2%2E1v8%2E59a%2E75%2E75%200%20001%2E5%200V4%2E66l1%2E95%202%2E1a%2E75%2E75%200%20101%2E1%2D1%2E02l%2D3%2E25%2D3%2E5a%2E75%2E75%200%2000%2D1%2E1%200L2%2E2%205%2E74a%2E75%2E75%200%2000%2E04%201%2E06zm8%206%2E4a%2E75%2E75%200%2000%2D%2E04%201%2E06l3%2E25%203%2E5a%2E75%2E75%200%20001%2E1%200l3%2E25%2D3%2E5a%2E75%2E75%200%2010%2D1%2E1%2D1%2E02l%2D1%2E95%202%2E1V6%2E75a%2E75%2E75%200%2000%2D1%2E5%200v8%2E59l%2D1%2E95%2D2%2E1a%2E75%2E75%200%2000%2D1%2E06%2D%2E04z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowsUpDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.24 6.8a.75.75 0 001.06-.04l1.95-2.1v8.59a.75.75 0 001.5 0V4.66l1.95 2.1a.75.75 0 101.1-1.02l-3.25-3.5a.75.75 0 00-1.1 0L2.2 5.74a.75.75 0 00.04 1.06zm8 6.4a.75.75 0 00-.04 1.06l3.25 3.5a.75.75 0 001.1 0l3.25-3.5a.75.75 0 10-1.1-1.02l-1.95 2.1V6.75a.75.75 0 00-1.5 0v8.59l-1.95-2.1a.75.75 0 00-1.06-.04z" clip-rule="evenodd"/>
</svg>
  }
}
