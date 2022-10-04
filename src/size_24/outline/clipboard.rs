use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2E666%203%2E888A2%2E25%202%2E25%200%200013%2E5%202%2E25h%2D3c%2D1%2E03%200%2D1%2E9%2E693%2D2%2E166%201%2E638m7%2E332%200c%2E055%2E194%2E084%2E4%2E084%2E612v0a%2E75%2E75%200%2001%2D%2E75%2E75H9a%2E75%2E75%200%2001%2D%2E75%2D%2E75v0c0%2D%2E212%2E03%2D%2E418%2E084%2D%2E612m7%2E332%200c%2E646%2E049%201%2E288%2E11%201%2E927%2E184%201%2E1%2E128%201%2E907%201%2E077%201%2E907%202%2E185V19%2E5a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25H6%2E75A2%2E25%202%2E25%200%20014%2E5%2019%2E5V6%2E257c0%2D1%2E108%2E806%2D2%2E057%201%2E907%2D2%2E185a48%2E208%2048%2E208%200%20011%2E927%2D%2E184%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ClipboardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184"/>
</svg>
  }
}