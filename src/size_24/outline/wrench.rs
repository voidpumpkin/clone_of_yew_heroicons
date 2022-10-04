use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2E75%206%2E75a4%2E5%204%2E5%200%2001%2D4%2E884%204%2E484c%2D1%2E076%2D%2E091%2D2%2E264%2E071%2D2%2E95%2E904l%2D7%2E152%208%2E684a2%2E548%202%2E548%200%2011%2D3%2E586%2D3%2E586l8%2E684%2D7%2E152c%2E833%2D%2E686%2E995%2D1%2E874%2E904%2D2%2E95a4%2E5%204%2E5%200%20016%2E336%2D4%2E486l%2D3%2E276%203%2E276a3%2E004%203%2E004%200%20002%2E25%202%2E25l3%2E276%2D3%2E276c%2E256%2E565%2E398%201%2E192%2E398%201%2E852z%22%2F%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M4%2E867%2019%2E125h%2E008v%2E008h%2D%2E008v%2D%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn WrenchIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75a4.5 4.5 0 01-4.884 4.484c-1.076-.091-2.264.071-2.95.904l-7.152 8.684a2.548 2.548 0 11-3.586-3.586l8.684-7.152c.833-.686.995-1.874.904-2.95a4.5 4.5 0 016.336-4.486l-3.276 3.276a3.004 3.004 0 002.25 2.25l3.276-3.276c.256.565.398 1.192.398 1.852z"/>
  <path stroke-linecap="round" stroke-linejoin="round" d="M4.867 19.125h.008v.008h-.008v-.008z"/>
</svg>
  }
}
