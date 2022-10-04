use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E5%204A1%2E5%201%2E5%200%20001%205%2E5V6h18v%2D%2E5A1%2E5%201%2E5%200%200017%2E5%204h%2D15zM19%208%2E5H1v6A1%2E5%201%2E5%200%20002%2E5%2016h15a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D6zM3%2013%2E25a%2E75%2E75%200%2001%2E75%2D%2E75h1%2E5a%2E75%2E75%200%20010%201%2E5h%2D1%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75zm4%2E75%2D%2E75a%2E75%2E75%200%20000%201%2E5h3%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D3%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CreditCardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.5 4A1.5 1.5 0 001 5.5V6h18v-.5A1.5 1.5 0 0017.5 4h-15zM19 8.5H1v6A1.5 1.5 0 002.5 16h15a1.5 1.5 0 001.5-1.5v-6zM3 13.25a.75.75 0 01.75-.75h1.5a.75.75 0 010 1.5h-1.5a.75.75 0 01-.75-.75zm4.75-.75a.75.75 0 000 1.5h3.5a.75.75 0 000-1.5h-3.5z" clip-rule="evenodd"/>
</svg>
  }
}