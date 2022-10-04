use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2E75%2010%2E5V6a3%2E75%203%2E75%200%2010%2D7%2E5%200v4%2E5m11%2E356%2D1%2E993l1%2E263%2012c%2E07%2E665%2D%2E45%201%2E243%2D1%2E119%201%2E243H4%2E25a1%2E125%201%2E125%200%2001%2D1%2E12%2D1%2E243l1%2E264%2D12A1%2E125%201%2E125%200%20015%2E513%207%2E5h12%2E974c%2E576%200%201%2E059%2E435%201%2E119%201%2E007zM8%2E625%2010%2E5a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200zm7%2E5%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShoppingBagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 10.5V6a3.75 3.75 0 10-7.5 0v4.5m11.356-1.993l1.263 12c.07.665-.45 1.243-1.119 1.243H4.25a1.125 1.125 0 01-1.12-1.243l1.264-12A1.125 1.125 0 015.513 7.5h12.974c.576 0 1.059.435 1.119 1.007zM8.625 10.5a.375.375 0 11-.75 0 .375.375 0 01.75 0zm7.5 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"/>
</svg>
  }
}