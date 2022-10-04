use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%207%2E5h1%2E5m%2D1%2E5%203h1%2E5m%2D7%2E5%203h7%2E5m%2D7%2E5%203h7%2E5m3%2D9h3%2E375c%2E621%200%201%2E125%2E504%201%2E125%201%2E125V18a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25M16%2E5%207%2E5V18a2%2E25%202%2E25%200%20002%2E25%202%2E25M16%2E5%207%2E5V4%2E875c0%2D%2E621%2D%2E504%2D1%2E125%2D1%2E125%2D1%2E125H4%2E125C3%2E504%203%2E75%203%204%2E254%203%204%2E875V18a2%2E25%202%2E25%200%20002%2E25%202%2E25h13%2E5M6%207%2E5h3v3H6v%2D3z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn NewspaperIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 7.5h1.5m-1.5 3h1.5m-7.5 3h7.5m-7.5 3h7.5m3-9h3.375c.621 0 1.125.504 1.125 1.125V18a2.25 2.25 0 01-2.25 2.25M16.5 7.5V18a2.25 2.25 0 002.25 2.25M16.5 7.5V4.875c0-.621-.504-1.125-1.125-1.125H4.125C3.504 3.75 3 4.254 3 4.875V18a2.25 2.25 0 002.25 2.25h13.5M6 7.5h3v3H6v-3z"/>
</svg>
  }
}