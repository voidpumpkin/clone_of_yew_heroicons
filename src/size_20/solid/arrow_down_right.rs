use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M6%2E28%205%2E22a%2E75%2E75%200%2000%2D1%2E06%201%2E06l7%2E22%207%2E22H6%2E75a%2E75%2E75%200%20000%201%2E5h7%2E5a%2E747%2E747%200%2000%2E75%2D%2E75v%2D7%2E5a%2E75%2E75%200%2000%2D1%2E5%200v5%2E69L6%2E28%205%2E22z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownRightIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M6.28 5.22a.75.75 0 00-1.06 1.06l7.22 7.22H6.75a.75.75 0 000 1.5h7.5a.747.747 0 00.75-.75v-7.5a.75.75 0 00-1.5 0v5.69L6.28 5.22z"/>
</svg>
  }
}
