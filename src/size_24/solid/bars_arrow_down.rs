use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%204%2E5A%2E75%2E75%200%20013%203%2E75h14%2E25a%2E75%2E75%200%20010%201%2E5H3a%2E75%2E75%200%2001%2D%2E75%2D%2E75zm0%204%2E5A%2E75%2E75%200%20013%208%2E25h9%2E75a%2E75%2E75%200%20010%201%2E5H3A%2E75%2E75%200%20012%2E25%209zm15%2D%2E75A%2E75%2E75%200%200118%209v10%2E19l2%2E47%2D2%2E47a%2E75%2E75%200%20111%2E06%201%2E06l%2D3%2E75%203%2E75a%2E75%2E75%200%2001%2D1%2E06%200l%2D3%2E75%2D3%2E75a%2E75%2E75%200%20111%2E06%2D1%2E06l2%2E47%202%2E47V9a%2E75%2E75%200%2001%2E75%2D%2E75zm%2D15%205%2E25a%2E75%2E75%200%2001%2E75%2D%2E75h9%2E75a%2E75%2E75%200%20010%201%2E5H3a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BarsArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 4.5A.75.75 0 013 3.75h14.25a.75.75 0 010 1.5H3a.75.75 0 01-.75-.75zm0 4.5A.75.75 0 013 8.25h9.75a.75.75 0 010 1.5H3A.75.75 0 012.25 9zm15-.75A.75.75 0 0118 9v10.19l2.47-2.47a.75.75 0 111.06 1.06l-3.75 3.75a.75.75 0 01-1.06 0l-3.75-3.75a.75.75 0 111.06-1.06l2.47 2.47V9a.75.75 0 01.75-.75zm-15 5.25a.75.75 0 01.75-.75h9.75a.75.75 0 010 1.5H3a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}
