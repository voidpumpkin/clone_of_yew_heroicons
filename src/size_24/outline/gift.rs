use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2011%2E25v8%2E25a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5H5%2E25a1%2E5%201%2E5%200%2001%2D1%2E5%2D1%2E5v%2D8%2E25M12%204%2E875A2%2E625%202%2E625%200%20109%2E375%207%2E5H12m0%2D2%2E625V7%2E5m0%2D2%2E625A2%2E625%202%2E625%200%201114%2E625%207%2E5H12m0%200V21m%2D8%2E625%2D9%2E75h18c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125v%2D1%2E5c0%2D%2E621%2D%2E504%2D1%2E125%2D1%2E125%2D1%2E125h%2D18c%2D%2E621%200%2D1%2E125%2E504%2D1%2E125%201%2E125v1%2E5c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn GiftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 11.25v8.25a1.5 1.5 0 01-1.5 1.5H5.25a1.5 1.5 0 01-1.5-1.5v-8.25M12 4.875A2.625 2.625 0 109.375 7.5H12m0-2.625V7.5m0-2.625A2.625 2.625 0 1114.625 7.5H12m0 0V21m-8.625-9.75h18c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125h-18c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125z"/>
</svg>
  }
}
