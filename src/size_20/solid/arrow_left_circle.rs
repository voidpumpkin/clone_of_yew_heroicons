use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cg%20clip%2Dpath%3D%22url%28%23clip0%5F9%5F2121%29%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2018a8%208%200%20100%2D16%208%208%200%20000%2016zm3%2E25%2D7%2E25a%2E75%2E75%200%20000%2D1%2E5H8%2E66l2%2E1%2D1%2E95a%2E75%2E75%200%2010%2D1%2E02%2D1%2E1l%2D3%2E5%203%2E25a%2E75%2E75%200%20000%201%2E1l3%2E5%203%2E25a%2E75%2E75%200%20001%2E02%2D1%2E1l%2D2%2E1%2D1%2E95h4%2E59z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fg%3E%20%3Cdefs%3E%20%3CclipPath%20id%3D%22clip0%5F9%5F2121%22%3E%20%3Cpath%20d%3D%22M0%200h20v20H0z%22%2F%3E%20%3C%2FclipPath%3E%20%3C%2Fdefs%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowLeftCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <g clip-path="url(#clip0_9_2121)">
    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.25-7.25a.75.75 0 000-1.5H8.66l2.1-1.95a.75.75 0 10-1.02-1.1l-3.5 3.25a.75.75 0 000 1.1l3.5 3.25a.75.75 0 001.02-1.1l-2.1-1.95h4.59z" clip-rule="evenodd"/>
  </g>
  <defs>
    <clipPath id="clip0_9_2121">
      <path d="M0 0h20v20H0z"/>
    </clipPath>
  </defs>
</svg>
  }
}
