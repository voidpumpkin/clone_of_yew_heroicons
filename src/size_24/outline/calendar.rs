use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M6%2E75%203v2%2E25M17%2E25%203v2%2E25M3%2018%2E75V7%2E5a2%2E25%202%2E25%200%20012%2E25%2D2%2E25h13%2E5A2%2E25%202%2E25%200%200121%207%2E5v11%2E25m%2D18%200A2%2E25%202%2E25%200%20005%2E25%2021h13%2E5A2%2E25%202%2E25%200%200021%2018%2E75m%2D18%200v%2D7%2E5A2%2E25%202%2E25%200%20015%2E25%209h13%2E5A2%2E25%202%2E25%200%200121%2011%2E25v7%2E5%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CalendarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"/>
</svg>
  }
}