use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%203h1%2E386c%2E51%200%20%2E955%2E343%201%2E087%2E835l%2E383%201%2E437M7%2E5%2014%2E25a3%203%200%2000%2D3%203h15%2E75m%2D12%2E75%2D3h11%2E218c1%2E121%2D2%2E3%202%2E1%2D4%2E684%202%2E924%2D7%2E138a60%2E114%2060%2E114%200%2000%2D16%2E536%2D1%2E84M7%2E5%2014%2E25L5%2E106%205%2E272M6%2020%2E25a%2E75%2E75%200%2011%2D1%2E5%200%20%2E75%2E75%200%20011%2E5%200zm12%2E75%200a%2E75%2E75%200%2011%2D1%2E5%200%20%2E75%2E75%200%20011%2E5%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShoppingCartIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 3h1.386c.51 0 .955.343 1.087.835l.383 1.437M7.5 14.25a3 3 0 00-3 3h15.75m-12.75-3h11.218c1.121-2.3 2.1-4.684 2.924-7.138a60.114 60.114 0 00-16.536-1.84M7.5 14.25L5.106 5.272M6 20.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm12.75 0a.75.75 0 11-1.5 0 .75.75 0 011.5 0z"/>
</svg>
  }
}