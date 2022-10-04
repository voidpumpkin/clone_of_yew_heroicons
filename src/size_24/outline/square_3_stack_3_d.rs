use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M6%2E429%209%2E75L2%2E25%2012l4%2E179%202%2E25m0%2D4%2E5l5%2E571%203%205%2E571%2D3m%2D11%2E142%200L2%2E25%207%2E5%2012%202%2E25l9%2E75%205%2E25%2D4%2E179%202%2E25m0%200L21%2E75%2012l%2D4%2E179%202%2E25m0%200l4%2E179%202%2E25L12%2021%2E75%202%2E25%2016%2E5l4%2E179%2D2%2E25m11%2E142%200l%2D5%2E571%203%2D5%2E571%2D3%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Square3Stack3DIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M6.429 9.75L2.25 12l4.179 2.25m0-4.5l5.571 3 5.571-3m-11.142 0L2.25 7.5 12 2.25l9.75 5.25-4.179 2.25m0 0L21.75 12l-4.179 2.25m0 0l4.179 2.25L12 21.75 2.25 16.5l4.179-2.25m11.142 0l-5.571 3-5.571-3"/>
</svg>
  }
}