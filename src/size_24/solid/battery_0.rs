use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M%2E75%209%2E75a3%203%200%20013%2D3h15a3%203%200%20013%203v%2E038c%2E856%2E173%201%2E5%2E93%201%2E5%201%2E837v2%2E25c0%20%2E907%2D%2E644%201%2E664%2D1%2E5%201%2E838v%2E037a3%203%200%2001%2D3%203h%2D15a3%203%200%2001%2D3%2D3v%2D6zm19%2E5%200a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5h%2D15a1%2E5%201%2E5%200%2000%2D1%2E5%201%2E5v6a1%2E5%201%2E5%200%20001%2E5%201%2E5h15a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D6z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Battery0Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M.75 9.75a3 3 0 013-3h15a3 3 0 013 3v.038c.856.173 1.5.93 1.5 1.837v2.25c0 .907-.644 1.664-1.5 1.838v.037a3 3 0 01-3 3h-15a3 3 0 01-3-3v-6zm19.5 0a1.5 1.5 0 00-1.5-1.5h-15a1.5 1.5 0 00-1.5 1.5v6a1.5 1.5 0 001.5 1.5h15a1.5 1.5 0 001.5-1.5v-6z" clip-rule="evenodd"/>
</svg>
  }
}
