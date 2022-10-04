use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%2012%2E76c0%201%2E6%201%2E123%202%2E994%202%2E707%203%2E227%201%2E087%2E16%202%2E185%2E283%203%2E293%2E369V21l4%2E076%2D4%2E076a1%2E526%201%2E526%200%20011%2E037%2D%2E443%2048%2E282%2048%2E282%200%20005%2E68%2D%2E494c1%2E584%2D%2E233%202%2E707%2D1%2E626%202%2E707%2D3%2E228V6%2E741c0%2D1%2E602%2D1%2E123%2D2%2E995%2D2%2E707%2D3%2E228A48%2E394%2048%2E394%200%200012%203c%2D2%2E392%200%2D4%2E744%2E175%2D7%2E043%2E513C3%2E373%203%2E746%202%2E25%205%2E14%202%2E25%206%2E741v6%2E018z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.76c0 1.6 1.123 2.994 2.707 3.227 1.087.16 2.185.283 3.293.369V21l4.076-4.076a1.526 1.526 0 011.037-.443 48.282 48.282 0 005.68-.494c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z"/>
</svg>
  }
}