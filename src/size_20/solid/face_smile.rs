use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2018a8%208%200%20100%2D16%208%208%200%20000%2016zm3%2E536%2D4%2E464a%2E75%2E75%200%2010%2D1%2E061%2D1%2E061%203%2E5%203%2E5%200%2001%2D4%2E95%200%20%2E75%2E75%200%2000%2D1%2E06%201%2E06%205%205%200%20007%2E07%200zM9%208%2E5c0%20%2E828%2D%2E448%201%2E5%2D1%201%2E5s%2D1%2D%2E672%2D1%2D1%2E5S7%2E448%207%208%207s1%20%2E672%201%201%2E5zm3%201%2E5c%2E552%200%201%2D%2E672%201%2D1%2E5S12%2E552%207%2012%207s%2D1%20%2E672%2D1%201%2E5%2E448%201%2E5%201%201%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FaceSmileIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.536-4.464a.75.75 0 10-1.061-1.061 3.5 3.5 0 01-4.95 0 .75.75 0 00-1.06 1.06 5 5 0 007.07 0zM9 8.5c0 .828-.448 1.5-1 1.5s-1-.672-1-1.5S7.448 7 8 7s1 .672 1 1.5zm3 1.5c.552 0 1-.672 1-1.5S12.552 7 12 7s-1 .672-1 1.5.448 1.5 1 1.5z" clip-rule="evenodd"/>
</svg>
  }
}
