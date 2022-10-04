use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%2015%2E75l5%2E159%2D5%2E159a2%2E25%202%2E25%200%20013%2E182%200l5%2E159%205%2E159m%2D1%2E5%2D1%2E5l1%2E409%2D1%2E409a2%2E25%202%2E25%200%20013%2E182%200l2%2E909%202%2E909m%2D18%203%2E75h16%2E5a1%2E5%201%2E5%200%20001%2E5%2D1%2E5V6a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5H3%2E75A1%2E5%201%2E5%200%20002%2E25%206v12a1%2E5%201%2E5%200%20001%2E5%201%2E5zm10%2E5%2D11%2E25h%2E008v%2E008h%2D%2E008V8%2E25zm%2E375%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PhotoIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"/>
</svg>
  }
}
