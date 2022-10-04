use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M7%2E5%203%2E75H6A2%2E25%202%2E25%200%20003%2E75%206v1%2E5M16%2E5%203%2E75H18A2%2E25%202%2E25%200%200120%2E25%206v1%2E5m0%209V18A2%2E25%202%2E25%200%200118%2020%2E25h%2D1%2E5m%2D9%200H6A2%2E25%202%2E25%200%20013%2E75%2018v%2D1%2E5M15%2012a3%203%200%2011%2D6%200%203%203%200%20016%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ViewfinderCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 3.75H6A2.25 2.25 0 003.75 6v1.5M16.5 3.75H18A2.25 2.25 0 0120.25 6v1.5m0 9V18A2.25 2.25 0 0118 20.25h-1.5m-9 0H6A2.25 2.25 0 013.75 18v-1.5M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
</svg>
  }
}
