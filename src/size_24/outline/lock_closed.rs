use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M16%2E5%2010%2E5V6%2E75a4%2E5%204%2E5%200%2010%2D9%200v3%2E75m%2D%2E75%2011%2E25h10%2E5a2%2E25%202%2E25%200%20002%2E25%2D2%2E25v%2D6%2E75a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H6%2E75a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25v6%2E75a2%2E25%202%2E25%200%20002%2E25%202%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LockClosedIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"/>
</svg>
  }
}