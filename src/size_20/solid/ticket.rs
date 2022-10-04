use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M13%203v1%2E27a%2E75%2E75%200%20001%2E5%200V3h2%2E25A2%2E25%202%2E25%200%200119%205%2E25v2%2E628a%2E75%2E75%200%2001%2D%2E5%2E707%201%2E5%201%2E5%200%20000%202%2E83c%2E3%2E106%2E5%2E39%2E5%2E707v2%2E628A2%2E25%202%2E25%200%200116%2E75%2017H14%2E5v%2D1%2E27a%2E75%2E75%200%2000%2D1%2E5%200V17H3%2E25A2%2E25%202%2E25%200%20011%2014%2E75v%2D2%2E628c0%2D%2E318%2E2%2D%2E601%2E5%2D%2E707a1%2E5%201%2E5%200%20000%2D2%2E83%2E75%2E75%200%2001%2D%2E5%2D%2E707V5%2E25A2%2E25%202%2E25%200%20013%2E25%203H13zm1%2E5%204%2E396a%2E75%2E75%200%2000%2D1%2E5%200v1%2E042a%2E75%2E75%200%20001%2E5%200V7%2E396zm0%204%2E167a%2E75%2E75%200%2000%2D1%2E5%200v1%2E041a%2E75%2E75%200%20001%2E5%200v%2D1%2E041zM6%2010%2E75a%2E75%2E75%200%2001%2E75%2D%2E75h3%2E5a%2E75%2E75%200%20010%201%2E5h%2D3%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75zm0%202%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h1%2E5a%2E75%2E75%200%20010%201%2E5h%2D1%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TicketIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M13 3v1.27a.75.75 0 001.5 0V3h2.25A2.25 2.25 0 0119 5.25v2.628a.75.75 0 01-.5.707 1.5 1.5 0 000 2.83c.3.106.5.39.5.707v2.628A2.25 2.25 0 0116.75 17H14.5v-1.27a.75.75 0 00-1.5 0V17H3.25A2.25 2.25 0 011 14.75v-2.628c0-.318.2-.601.5-.707a1.5 1.5 0 000-2.83.75.75 0 01-.5-.707V5.25A2.25 2.25 0 013.25 3H13zm1.5 4.396a.75.75 0 00-1.5 0v1.042a.75.75 0 001.5 0V7.396zm0 4.167a.75.75 0 00-1.5 0v1.041a.75.75 0 001.5 0v-1.041zM6 10.75a.75.75 0 01.75-.75h3.5a.75.75 0 010 1.5h-3.5a.75.75 0 01-.75-.75zm0 2.5a.75.75 0 01.75-.75h1.5a.75.75 0 010 1.5h-1.5a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}