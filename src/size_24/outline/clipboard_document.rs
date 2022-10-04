use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E25%207%2E5V6%2E108c0%2D1%2E135%2E845%2D2%2E098%201%2E976%2D2%2E192%2E373%2D%2E03%2E748%2D%2E057%201%2E123%2D%2E08M15%2E75%2018H18a2%2E25%202%2E25%200%20002%2E25%2D2%2E25V6%2E108c0%2D1%2E135%2D%2E845%2D2%2E098%2D1%2E976%2D2%2E192a48%2E424%2048%2E424%200%2000%2D1%2E123%2D%2E08M15%2E75%2018%2E75v%2D1%2E875a3%2E375%203%2E375%200%2000%2D3%2E375%2D3%2E375h%2D1%2E5a1%2E125%201%2E125%200%2001%2D1%2E125%2D1%2E125v%2D1%2E5A3%2E375%203%2E375%200%20006%2E375%207%2E5H5%2E25m11%2E9%2D3%2E664A2%2E251%202%2E251%200%200015%202%2E25h%2D1%2E5a2%2E251%202%2E251%200%2000%2D2%2E15%201%2E586m5%2E8%200c%2E065%2E21%2E1%2E433%2E1%2E664v%2E75h%2D6V4%2E5c0%2D%2E231%2E035%2D%2E454%2E1%2D%2E664M6%2E75%207%2E5H4%2E875c%2D%2E621%200%2D1%2E125%2E504%2D1%2E125%201%2E125v12c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125h9%2E75c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125V16%2E5a9%209%200%2000%2D9%2D9z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ClipboardDocumentIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 7.5V6.108c0-1.135.845-2.098 1.976-2.192.373-.03.748-.057 1.123-.08M15.75 18H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08M15.75 18.75v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5A3.375 3.375 0 006.375 7.5H5.25m11.9-3.664A2.251 2.251 0 0015 2.25h-1.5a2.251 2.251 0 00-2.15 1.586m5.8 0c.065.21.1.433.1.664v.75h-6V4.5c0-.231.035-.454.1-.664M6.75 7.5H4.875c-.621 0-1.125.504-1.125 1.125v12c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V16.5a9 9 0 00-9-9z"/>
</svg>
  }
}