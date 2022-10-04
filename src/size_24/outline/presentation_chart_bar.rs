use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%2E75%203v11%2E25A2%2E25%202%2E25%200%20006%2016%2E5h2%2E25M3%2E75%203h%2D1%2E5m1%2E5%200h16%2E5m0%200h1%2E5m%2D1%2E5%200v11%2E25A2%2E25%202%2E25%200%200118%2016%2E5h%2D2%2E25m%2D7%2E5%200h7%2E5m%2D7%2E5%200l%2D1%203m8%2E5%2D3l1%203m0%200l%2E5%201%2E5m%2D%2E5%2D1%2E5h%2D9%2E5m0%200l%2D%2E5%201%2E5M9%2011%2E25v1%2E5M12%209v3%2E75m3%2D6v6%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PresentationChartBarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3v11.25A2.25 2.25 0 006 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0118 16.5h-2.25m-7.5 0h7.5m-7.5 0l-1 3m8.5-3l1 3m0 0l.5 1.5m-.5-1.5h-9.5m0 0l-.5 1.5M9 11.25v1.5M12 9v3.75m3-6v6"/>
</svg>
  }
}
