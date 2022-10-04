use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%202%2E25a%2E75%2E75%200%20000%201%2E5H3v10%2E5a3%203%200%20003%203h1%2E21l%2D1%2E172%203%2E513a%2E75%2E75%200%20001%2E424%2E474l%2E329%2D%2E987h8%2E418l%2E33%2E987a%2E75%2E75%200%20001%2E422%2D%2E474l%2D1%2E17%2D3%2E513H18a3%203%200%20003%2D3V3%2E75h%2E75a%2E75%2E75%200%20000%2D1%2E5H2%2E25zm6%2E04%2016%2E5l%2E5%2D1%2E5h6%2E42l%2E5%201%2E5H8%2E29zm7%2E46%2D12a%2E75%2E75%200%2000%2D1%2E5%200v6a%2E75%2E75%200%20001%2E5%200v%2D6zm%2D3%202%2E25a%2E75%2E75%200%2000%2D1%2E5%200v3%2E75a%2E75%2E75%200%20001%2E5%200V9zm%2D3%202%2E25a%2E75%2E75%200%2000%2D1%2E5%200v1%2E5a%2E75%2E75%200%20001%2E5%200v%2D1%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PresentationChartBarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 2.25a.75.75 0 000 1.5H3v10.5a3 3 0 003 3h1.21l-1.172 3.513a.75.75 0 001.424.474l.329-.987h8.418l.33.987a.75.75 0 001.422-.474l-1.17-3.513H18a3 3 0 003-3V3.75h.75a.75.75 0 000-1.5H2.25zm6.04 16.5l.5-1.5h6.42l.5 1.5H8.29zm7.46-12a.75.75 0 00-1.5 0v6a.75.75 0 001.5 0v-6zm-3 2.25a.75.75 0 00-1.5 0v3.75a.75.75 0 001.5 0V9zm-3 2.25a.75.75 0 00-1.5 0v1.5a.75.75 0 001.5 0v-1.5z" clip-rule="evenodd"/>
</svg>
  }
}
