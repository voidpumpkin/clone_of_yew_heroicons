use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2012%2E75L11%2E25%2015%2015%209%2E75m%2D3%2D7%2E036A11%2E959%2011%2E959%200%20013%2E598%206%2011%2E99%2011%2E99%200%20003%209%2E749c0%205%2E592%203%2E824%2010%2E29%209%2011%2E623%205%2E176%2D1%2E332%209%2D6%2E03%209%2D11%2E622%200%2D1%2E31%2D%2E21%2D2%2E571%2D%2E598%2D3%2E751h%2D%2E152c%2D3%2E196%200%2D6%2E1%2D1%2E248%2D8%2E25%2D3%2E285z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShieldCheckIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"/>
</svg>
  }
}