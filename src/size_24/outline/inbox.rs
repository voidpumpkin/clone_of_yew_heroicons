use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%2013%2E5h3%2E86a2%2E25%202%2E25%200%20012%2E012%201%2E244l%2E256%2E512a2%2E25%202%2E25%200%20002%2E013%201%2E244h3%2E218a2%2E25%202%2E25%200%20002%2E013%2D1%2E244l%2E256%2D%2E512a2%2E25%202%2E25%200%20012%2E013%2D1%2E244h3%2E859m%2D19%2E5%2E338V18a2%2E25%202%2E25%200%20002%2E25%202%2E25h15A2%2E25%202%2E25%200%200021%2E75%2018v%2D4%2E162c0%2D%2E224%2D%2E034%2D%2E447%2D%2E1%2D%2E661L19%2E24%205%2E338a2%2E25%202%2E25%200%2000%2D2%2E15%2D1%2E588H6%2E911a2%2E25%202%2E25%200%2000%2D2%2E15%201%2E588L2%2E35%2013%2E177a2%2E25%202%2E25%200%2000%2D%2E1%2E661z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn InboxIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 13.5h3.86a2.25 2.25 0 012.012 1.244l.256.512a2.25 2.25 0 002.013 1.244h3.218a2.25 2.25 0 002.013-1.244l.256-.512a2.25 2.25 0 012.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 00-2.15-1.588H6.911a2.25 2.25 0 00-2.15 1.588L2.35 13.177a2.25 2.25 0 00-.1.661z"/>
</svg>
  }
}
