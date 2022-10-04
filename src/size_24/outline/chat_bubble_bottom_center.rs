use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%2012%2E76c0%201%2E6%201%2E123%202%2E994%202%2E707%203%2E227%201%2E068%2E157%202%2E148%2E279%203%2E238%2E364%2E466%2E037%2E893%2E281%201%2E153%2E671L12%2021l2%2E652%2D3%2E978c%2E26%2D%2E39%2E687%2D%2E634%201%2E153%2D%2E67%201%2E09%2D%2E086%202%2E17%2D%2E208%203%2E238%2D%2E365%201%2E584%2D%2E233%202%2E707%2D1%2E626%202%2E707%2D3%2E228V6%2E741c0%2D1%2E602%2D1%2E123%2D2%2E995%2D2%2E707%2D3%2E228A48%2E394%2048%2E394%200%200012%203c%2D2%2E392%200%2D4%2E744%2E175%2D7%2E043%2E513C3%2E373%203%2E746%202%2E25%205%2E14%202%2E25%206%2E741v6%2E018z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleBottomCenterIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.76c0 1.6 1.123 2.994 2.707 3.227 1.068.157 2.148.279 3.238.364.466.037.893.281 1.153.671L12 21l2.652-3.978c.26-.39.687-.634 1.153-.67 1.09-.086 2.17-.208 3.238-.365 1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z"/>
</svg>
  }
}
