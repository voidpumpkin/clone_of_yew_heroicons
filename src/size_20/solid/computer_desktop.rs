use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%204%2E25A2%2E25%202%2E25%200%20014%2E25%202h11%2E5A2%2E25%202%2E25%200%200118%204%2E25v8%2E5A2%2E25%202%2E25%200%200115%2E75%2015h%2D3%2E105a3%2E501%203%2E501%200%20001%2E1%201%2E677A%2E75%2E75%200%200113%2E26%2018H6%2E74a%2E75%2E75%200%2001%2D%2E484%2D1%2E323A3%2E501%203%2E501%200%20007%2E355%2015H4%2E25A2%2E25%202%2E25%200%20012%2012%2E75v%2D8%2E5zm1%2E5%200a%2E75%2E75%200%2001%2E75%2D%2E75h11%2E5a%2E75%2E75%200%2001%2E75%2E75v7%2E5a%2E75%2E75%200%2001%2D%2E75%2E75H4%2E25a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D7%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ComputerDesktopIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2 4.25A2.25 2.25 0 014.25 2h11.5A2.25 2.25 0 0118 4.25v8.5A2.25 2.25 0 0115.75 15h-3.105a3.501 3.501 0 001.1 1.677A.75.75 0 0113.26 18H6.74a.75.75 0 01-.484-1.323A3.501 3.501 0 007.355 15H4.25A2.25 2.25 0 012 12.75v-8.5zm1.5 0a.75.75 0 01.75-.75h11.5a.75.75 0 01.75.75v7.5a.75.75 0 01-.75.75H4.25a.75.75 0 01-.75-.75v-7.5z" clip-rule="evenodd"/>
</svg>
  }
}
