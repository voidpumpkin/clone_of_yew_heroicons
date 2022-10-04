use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%2E5%206%2E375c0%2D1%2E036%2E84%2D1%2E875%201%2E875%2D1%2E875h17%2E25c1%2E035%200%201%2E875%2E84%201%2E875%201%2E875v3%2E026a%2E75%2E75%200%2001%2D%2E375%2E65%202%2E249%202%2E249%200%20000%203%2E898%2E75%2E75%200%2001%2E375%2E65v3%2E026c0%201%2E035%2D%2E84%201%2E875%2D1%2E875%201%2E875H3%2E375A1%2E875%201%2E875%200%20011%2E5%2017%2E625v%2D3%2E026a%2E75%2E75%200%2001%2E374%2D%2E65%202%2E249%202%2E249%200%20000%2D3%2E898%2E75%2E75%200%2001%2D%2E374%2D%2E65V6%2E375zm15%2D1%2E125a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D1%2E5%200V6a%2E75%2E75%200%2001%2E75%2D%2E75zm%2E75%204%2E5a%2E75%2E75%200%2000%2D1%2E5%200v%2E75a%2E75%2E75%200%20001%2E5%200v%2D%2E75zm%2D%2E75%203a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D1%2E5%200v%2D%2E75a%2E75%2E75%200%2001%2E75%2D%2E75zm%2E75%204%2E5a%2E75%2E75%200%2000%2D1%2E5%200V18a%2E75%2E75%200%20001%2E5%200v%2D%2E75zM6%2012a%2E75%2E75%200%2001%2E75%2D%2E75H12a%2E75%2E75%200%20010%201%2E5H6%2E75A%2E75%2E75%200%20016%2012zm%2E75%202%2E25a%2E75%2E75%200%20000%201%2E5h3a%2E75%2E75%200%20000%2D1%2E5h%2D3z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TicketIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1.5 6.375c0-1.036.84-1.875 1.875-1.875h17.25c1.035 0 1.875.84 1.875 1.875v3.026a.75.75 0 01-.375.65 2.249 2.249 0 000 3.898.75.75 0 01.375.65v3.026c0 1.035-.84 1.875-1.875 1.875H3.375A1.875 1.875 0 011.5 17.625v-3.026a.75.75 0 01.374-.65 2.249 2.249 0 000-3.898.75.75 0 01-.374-.65V6.375zm15-1.125a.75.75 0 01.75.75v.75a.75.75 0 01-1.5 0V6a.75.75 0 01.75-.75zm.75 4.5a.75.75 0 00-1.5 0v.75a.75.75 0 001.5 0v-.75zm-.75 3a.75.75 0 01.75.75v.75a.75.75 0 01-1.5 0v-.75a.75.75 0 01.75-.75zm.75 4.5a.75.75 0 00-1.5 0V18a.75.75 0 001.5 0v-.75zM6 12a.75.75 0 01.75-.75H12a.75.75 0 010 1.5H6.75A.75.75 0 016 12zm.75 2.25a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z" clip-rule="evenodd"/>
</svg>
  }
}