use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M19%2E5%2021a3%203%200%20003%2D3V9a3%203%200%2000%2D3%2D3h%2D5%2E379a%2E75%2E75%200%2001%2D%2E53%2D%2E22L11%2E47%203%2E66A2%2E25%202%2E25%200%20009%2E879%203H4%2E5a3%203%200%2000%2D3%203v12a3%203%200%20003%203h15zm%2D6%2E75%2D10%2E5a%2E75%2E75%200%2000%2D1%2E5%200v4%2E19l%2D1%2E72%2D1%2E72a%2E75%2E75%200%2000%2D1%2E06%201%2E06l3%203a%2E75%2E75%200%20001%2E06%200l3%2D3a%2E75%2E75%200%2010%2D1%2E06%2D1%2E06l%2D1%2E72%201%2E72V10%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FolderArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M19.5 21a3 3 0 003-3V9a3 3 0 00-3-3h-5.379a.75.75 0 01-.53-.22L11.47 3.66A2.25 2.25 0 009.879 3H4.5a3 3 0 00-3 3v12a3 3 0 003 3h15zm-6.75-10.5a.75.75 0 00-1.5 0v4.19l-1.72-1.72a.75.75 0 00-1.06 1.06l3 3a.75.75 0 001.06 0l3-3a.75.75 0 10-1.06-1.06l-1.72 1.72V10.5z" clip-rule="evenodd"/>
</svg>
  }
}
