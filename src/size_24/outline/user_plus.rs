use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M19%207%2E5v3m0%200v3m0%2D3h3m%2D3%200h%2D3m%2D2%2E25%2D4%2E125a3%2E375%203%2E375%200%2011%2D6%2E75%200%203%2E375%203%2E375%200%20016%2E75%200zM4%2019%2E235v%2D%2E11a6%2E375%206%2E375%200%200112%2E75%200v%2E109A12%2E318%2012%2E318%200%200110%2E374%2021c%2D2%2E331%200%2D4%2E512%2D%2E645%2D6%2E374%2D1%2E766z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserPlusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z"/>
</svg>
  }
}