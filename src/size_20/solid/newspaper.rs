use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%203%2E5A1%2E5%201%2E5%200%20013%2E5%202h9A1%2E5%201%2E5%200%200114%203%2E5v11%2E75A2%2E75%202%2E75%200%200016%2E75%2018h%2D12A2%2E75%202%2E75%200%20012%2015%2E25V3%2E5zm3%2E75%207a%2E75%2E75%200%20000%201%2E5h4%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D4%2E5zm0%203a%2E75%2E75%200%20000%201%2E5h4%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D4%2E5zM5%205%2E75A%2E75%2E75%200%20015%2E75%205h4%2E5a%2E75%2E75%200%2001%2E75%2E75v2%2E5a%2E75%2E75%200%2001%2D%2E75%2E75h%2D4%2E5A%2E75%2E75%200%20015%208%2E25v%2D2%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M16%2E5%206%2E5h%2D1v8%2E75a1%2E25%201%2E25%200%20102%2E5%200V8a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn NewspaperIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2 3.5A1.5 1.5 0 013.5 2h9A1.5 1.5 0 0114 3.5v11.75A2.75 2.75 0 0016.75 18h-12A2.75 2.75 0 012 15.25V3.5zm3.75 7a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zm0 3a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zM5 5.75A.75.75 0 015.75 5h4.5a.75.75 0 01.75.75v2.5a.75.75 0 01-.75.75h-4.5A.75.75 0 015 8.25v-2.5z" clip-rule="evenodd"/>
  <path d="M16.5 6.5h-1v8.75a1.25 1.25 0 102.5 0V8a1.5 1.5 0 00-1.5-1.5z"/>
</svg>
  }
}