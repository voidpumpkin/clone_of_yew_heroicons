use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%206a3%203%200%20013%2D3h12a3%203%200%20013%203v12a3%203%200%2001%2D3%203H6a3%203%200%2001%2D3%2D3V6zm4%2E5%207%2E5a%2E75%2E75%200%2001%2E75%2E75v2%2E25a%2E75%2E75%200%2001%2D1%2E5%200v%2D2%2E25a%2E75%2E75%200%2001%2E75%2D%2E75zm3%2E75%2D1%2E5a%2E75%2E75%200%2000%2D1%2E5%200v4%2E5a%2E75%2E75%200%20001%2E5%200V12zm2%2E25%2D3a%2E75%2E75%200%2001%2E75%2E75v6%2E75a%2E75%2E75%200%2001%2D1%2E5%200V9%2E75A%2E75%2E75%200%200113%2E5%209zm3%2E75%2D1%2E5a%2E75%2E75%200%2000%2D1%2E5%200v9a%2E75%2E75%200%20001%2E5%200v%2D9z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChartBarSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 6a3 3 0 013-3h12a3 3 0 013 3v12a3 3 0 01-3 3H6a3 3 0 01-3-3V6zm4.5 7.5a.75.75 0 01.75.75v2.25a.75.75 0 01-1.5 0v-2.25a.75.75 0 01.75-.75zm3.75-1.5a.75.75 0 00-1.5 0v4.5a.75.75 0 001.5 0V12zm2.25-3a.75.75 0 01.75.75v6.75a.75.75 0 01-1.5 0V9.75A.75.75 0 0113.5 9zm3.75-1.5a.75.75 0 00-1.5 0v9a.75.75 0 001.5 0v-9z" clip-rule="evenodd"/>
</svg>
  }
}