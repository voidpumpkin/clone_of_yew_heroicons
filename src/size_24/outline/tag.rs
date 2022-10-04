use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2E568%203H5%2E25A2%2E25%202%2E25%200%20003%205%2E25v4%2E318c0%20%2E597%2E237%201%2E17%2E659%201%2E591l9%2E581%209%2E581c%2E699%2E699%201%2E78%2E872%202%2E607%2E33a18%2E095%2018%2E095%200%20005%2E223%2D5%2E223c%2E542%2D%2E827%2E369%2D1%2E908%2D%2E33%2D2%2E607L11%2E16%203%2E66A2%2E25%202%2E25%200%20009%2E568%203z%22%2F%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M6%206h%2E008v%2E008H6V6z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z"/>
  <path stroke-linecap="round" stroke-linejoin="round" d="M6 6h.008v.008H6V6z"/>
</svg>
  }
}