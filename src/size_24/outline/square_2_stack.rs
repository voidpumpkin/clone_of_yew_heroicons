use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M16%2E5%208%2E25V6a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H6A2%2E25%202%2E25%200%20003%2E75%206v8%2E25A2%2E25%202%2E25%200%20006%2016%2E5h2%2E25m8%2E25%2D8%2E25H18a2%2E25%202%2E25%200%20012%2E25%202%2E25V18A2%2E25%202%2E25%200%200118%2020%2E25h%2D7%2E5A2%2E25%202%2E25%200%20018%2E25%2018v%2D1%2E5m8%2E25%2D8%2E25h%2D6a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25v6%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Square2StackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 8.25V6a2.25 2.25 0 00-2.25-2.25H6A2.25 2.25 0 003.75 6v8.25A2.25 2.25 0 006 16.5h2.25m8.25-8.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-7.5A2.25 2.25 0 018.25 18v-1.5m8.25-8.25h-6a2.25 2.25 0 00-2.25 2.25v6"/>
</svg>
  }
}
