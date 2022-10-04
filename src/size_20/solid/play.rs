use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M6%2E3%202%2E841A1%2E5%201%2E5%200%20004%204%2E11V15%2E89a1%2E5%201%2E5%200%20002%2E3%201%2E269l9%2E344%2D5%2E89a1%2E5%201%2E5%200%20000%2D2%2E538L6%2E3%202%2E84z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PlayIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M6.3 2.841A1.5 1.5 0 004 4.11V15.89a1.5 1.5 0 002.3 1.269l9.344-5.89a1.5 1.5 0 000-2.538L6.3 2.84z"/>
</svg>
  }
}
