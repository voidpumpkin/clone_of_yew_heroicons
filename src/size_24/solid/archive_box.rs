use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E375%203C2%2E339%203%201%2E5%203%2E84%201%2E5%204%2E875v%2E75c0%201%2E036%2E84%201%2E875%201%2E875%201%2E875h17%2E25c1%2E035%200%201%2E875%2D%2E84%201%2E875%2D1%2E875v%2D%2E75C22%2E5%203%2E839%2021%2E66%203%2020%2E625%203H3%2E375z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E087%209l%2E54%209%2E176A3%203%200%20006%2E62%2021h10%2E757a3%203%200%20002%2E995%2D2%2E824L20%2E913%209H3%2E087zm6%2E163%203%2E75A%2E75%2E75%200%200110%2012h4a%2E75%2E75%200%20010%201%2E5h%2D4a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArchiveBoxIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M3.375 3C2.339 3 1.5 3.84 1.5 4.875v.75c0 1.036.84 1.875 1.875 1.875h17.25c1.035 0 1.875-.84 1.875-1.875v-.75C22.5 3.839 21.66 3 20.625 3H3.375z"/>
  <path fill-rule="evenodd" d="M3.087 9l.54 9.176A3 3 0 006.62 21h10.757a3 3 0 002.995-2.824L20.913 9H3.087zm6.163 3.75A.75.75 0 0110 12h4a.75.75 0 010 1.5h-4a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}
