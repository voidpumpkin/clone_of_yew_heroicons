use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M11%2E983%201%2E907a%2E75%2E75%200%2000%2D1%2E292%2D%2E657l%2D8%2E5%209%2E5A%2E75%2E75%200%20002%2E75%2012h6%2E572l%2D1%2E305%206%2E093a%2E75%2E75%200%20001%2E292%2E657l8%2E5%2D9%2E5A%2E75%2E75%200%200017%2E25%208h%2D6%2E572l1%2E305%2D6%2E093z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BoltIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M11.983 1.907a.75.75 0 00-1.292-.657l-8.5 9.5A.75.75 0 002.75 12h6.572l-1.305 6.093a.75.75 0 001.292.657l8.5-9.5A.75.75 0 0017.25 8h-6.572l1.305-6.093z"/>
</svg>
  }
}
