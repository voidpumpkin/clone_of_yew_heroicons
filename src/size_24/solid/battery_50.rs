use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E5%209%2E75a%2E75%2E75%200%2000%2D%2E75%2E75V15c0%20%2E414%2E336%2E75%2E75%2E75h6%2E75A%2E75%2E75%200%200012%2015v%2D4%2E5a%2E75%2E75%200%2000%2D%2E75%2D%2E75H4%2E5z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E75%206%2E75a3%203%200%2000%2D3%203v6a3%203%200%20003%203h15a3%203%200%20003%2D3v%2D%2E037c%2E856%2D%2E174%201%2E5%2D%2E93%201%2E5%2D1%2E838v%2D2%2E25c0%2D%2E907%2D%2E644%2D1%2E664%2D1%2E5%2D1%2E837V9%2E75a3%203%200%2000%2D3%2D3h%2D15zm15%201%2E5a1%2E5%201%2E5%200%20011%2E5%201%2E5v6a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5h%2D15a1%2E5%201%2E5%200%2001%2D1%2E5%2D1%2E5v%2D6a1%2E5%201%2E5%200%20011%2E5%2D1%2E5h15z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Battery50Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M4.5 9.75a.75.75 0 00-.75.75V15c0 .414.336.75.75.75h6.75A.75.75 0 0012 15v-4.5a.75.75 0 00-.75-.75H4.5z"/>
  <path fill-rule="evenodd" d="M3.75 6.75a3 3 0 00-3 3v6a3 3 0 003 3h15a3 3 0 003-3v-.037c.856-.174 1.5-.93 1.5-1.838v-2.25c0-.907-.644-1.664-1.5-1.837V9.75a3 3 0 00-3-3h-15zm15 1.5a1.5 1.5 0 011.5 1.5v6a1.5 1.5 0 01-1.5 1.5h-15a1.5 1.5 0 01-1.5-1.5v-6a1.5 1.5 0 011.5-1.5h15z" clip-rule="evenodd"/>
</svg>
  }
}
