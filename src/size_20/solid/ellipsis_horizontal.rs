use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2010a1%2E5%201%2E5%200%20113%200%201%2E5%201%2E5%200%2001%2D3%200zM8%2E5%2010a1%2E5%201%2E5%200%20113%200%201%2E5%201%2E5%200%2001%2D3%200zM15%2E5%208%2E5a1%2E5%201%2E5%200%20100%203%201%2E5%201%2E5%200%20000%2D3z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EllipsisHorizontalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M3 10a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0zM8.5 10a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0zM15.5 8.5a1.5 1.5 0 100 3 1.5 1.5 0 000-3z"/>
</svg>
  }
}
