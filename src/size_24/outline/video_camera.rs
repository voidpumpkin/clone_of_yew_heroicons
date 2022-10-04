use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20d%3D%22M15%2E75%2010%2E5l4%2E72%2D4%2E72a%2E75%2E75%200%20011%2E28%2E53v11%2E38a%2E75%2E75%200%2001%2D1%2E28%2E53l%2D4%2E72%2D4%2E72M4%2E5%2018%2E75h9a2%2E25%202%2E25%200%20002%2E25%2D2%2E25v%2D9a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25h%2D9A2%2E25%202%2E25%200%20002%2E25%207%2E5v9a2%2E25%202%2E25%200%20002%2E25%202%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn VideoCameraIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" d="M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25h-9A2.25 2.25 0 002.25 7.5v9a2.25 2.25 0 002.25 2.25z"/>
</svg>
  }
}
