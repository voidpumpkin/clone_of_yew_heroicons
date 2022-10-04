use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E25%209%2E75h4%2E875a2%2E625%202%2E625%200%20010%205%2E25H12M8%2E25%209%2E75L10%2E5%207%2E5M8%2E25%209%2E75L10%2E5%2012m9%2D7%2E243V21%2E75l%2D3%2E75%2D1%2E5%2D3%2E75%201%2E5%2D3%2E75%2D1%2E5%2D3%2E75%201%2E5V4%2E757c0%2D1%2E108%2E806%2D2%2E057%201%2E907%2D2%2E185a48%2E507%2048%2E507%200%200111%2E186%200c1%2E1%2E128%201%2E907%201%2E077%201%2E907%202%2E185z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ReceiptRefundIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 9.75h4.875a2.625 2.625 0 010 5.25H12M8.25 9.75L10.5 7.5M8.25 9.75L10.5 12m9-7.243V21.75l-3.75-1.5-3.75 1.5-3.75-1.5-3.75 1.5V4.757c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0111.186 0c1.1.128 1.907 1.077 1.907 2.185z"/>
</svg>
  }
}
