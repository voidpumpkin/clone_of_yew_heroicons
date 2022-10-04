use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2E348%2014%2E651a3%2E75%203%2E75%200%20010%2D5%2E303m5%2E304%200a3%2E75%203%2E75%200%20010%205%2E303m%2D7%2E425%202%2E122a6%2E75%206%2E75%200%20010%2D9%2E546m9%2E546%200a6%2E75%206%2E75%200%20010%209%2E546M5%2E106%2018%2E894c%2D3%2E808%2D3%2E808%2D3%2E808%2D9%2E98%200%2D13%2E789m13%2E788%200c3%2E808%203%2E808%203%2E808%209%2E981%200%2013%2E79M12%2012h%2E008v%2E007H12V12zm%2E375%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SignalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9.348 14.651a3.75 3.75 0 010-5.303m5.304 0a3.75 3.75 0 010 5.303m-7.425 2.122a6.75 6.75 0 010-9.546m9.546 0a6.75 6.75 0 010 9.546M5.106 18.894c-3.808-3.808-3.808-9.98 0-13.789m13.788 0c3.808 3.808 3.808 9.981 0 13.79M12 12h.008v.007H12V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"/>
</svg>
  }
}