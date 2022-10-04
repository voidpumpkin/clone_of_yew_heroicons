use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M7%2E5%208%2E25h9m%2D9%203H12m%2D9%2E75%201%2E51c0%201%2E6%201%2E123%202%2E994%202%2E707%203%2E227%201%2E129%2E166%202%2E27%2E293%203%2E423%2E379%2E35%2E026%2E67%2E21%2E865%2E501L12%2021l2%2E755%2D4%2E133a1%2E14%201%2E14%200%2001%2E865%2D%2E501%2048%2E172%2048%2E172%200%20003%2E423%2D%2E379c1%2E584%2D%2E233%202%2E707%2D1%2E626%202%2E707%2D3%2E228V6%2E741c0%2D1%2E602%2D1%2E123%2D2%2E995%2D2%2E707%2D3%2E228A48%2E394%2048%2E394%200%200012%203c%2D2%2E392%200%2D4%2E744%2E175%2D7%2E043%2E513C3%2E373%203%2E746%202%2E25%205%2E14%202%2E25%206%2E741v6%2E018z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleBottomCenterTextIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z"/>
</svg>
  }
}
