use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%2012%2E75V12A2%2E25%202%2E25%200%20014%2E5%209%2E75h15A2%2E25%202%2E25%200%200121%2E75%2012v%2E75m%2D8%2E69%2D6%2E44l%2D2%2E12%2D2%2E12a1%2E5%201%2E5%200%2000%2D1%2E061%2D%2E44H4%2E5A2%2E25%202%2E25%200%20002%2E25%206v12a2%2E25%202%2E25%200%20002%2E25%202%2E25h15A2%2E25%202%2E25%200%200021%2E75%2018V9a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25h%2D5%2E379a1%2E5%201%2E5%200%2001%2D1%2E06%2D%2E44z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FolderIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"/>
</svg>
  }
}
