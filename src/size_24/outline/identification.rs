use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%209h3%2E75M15%2012h3%2E75M15%2015h3%2E75M4%2E5%2019%2E5h15a2%2E25%202%2E25%200%20002%2E25%2D2%2E25V6%2E75A2%2E25%202%2E25%200%200019%2E5%204%2E5h%2D15a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25v10%2E5A2%2E25%202%2E25%200%20004%2E5%2019%2E5zm6%2D10%2E125a1%2E875%201%2E875%200%2011%2D3%2E75%200%201%2E875%201%2E875%200%20013%2E75%200zm1%2E294%206%2E336a6%2E721%206%2E721%200%2001%2D3%2E17%2E789%206%2E721%206%2E721%200%2001%2D3%2E168%2D%2E789%203%2E376%203%2E376%200%20016%2E338%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn IdentificationIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15 9h3.75M15 12h3.75M15 15h3.75M4.5 19.5h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5zm6-10.125a1.875 1.875 0 11-3.75 0 1.875 1.875 0 013.75 0zm1.294 6.336a6.721 6.721 0 01-3.17.789 6.721 6.721 0 01-3.168-.789 3.376 3.376 0 016.338 0z"/>
</svg>
  }
}