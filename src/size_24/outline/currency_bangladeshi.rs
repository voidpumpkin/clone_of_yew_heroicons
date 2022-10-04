use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E25%207%2E5l%2E415%2D%2E207a%2E75%2E75%200%20011%2E085%2E67V10%2E5m0%200h6m%2D6%200h%2D1%2E5m1%2E5%200v5%2E438c0%20%2E354%2E161%2E697%2E473%2E865a3%2E751%203%2E751%200%20005%2E452%2D2%2E553c%2E083%2D%2E409%2D%2E263%2D%2E75%2D%2E68%2D%2E75h%2D%2E745M21%2012a9%209%200%2011%2D18%200%209%209%200%200118%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CurrencyBangladeshiIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 7.5l.415-.207a.75.75 0 011.085.67V10.5m0 0h6m-6 0h-1.5m1.5 0v5.438c0 .354.161.697.473.865a3.751 3.751 0 005.452-2.553c.083-.409-.263-.75-.68-.75h-.745M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
</svg>
  }
}
