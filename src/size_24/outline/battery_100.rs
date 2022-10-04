use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2010%2E5h%2E375c%2E621%200%201%2E125%2E504%201%2E125%201%2E125v2%2E25c0%20%2E621%2D%2E504%201%2E125%2D1%2E125%201%2E125H21M4%2E5%2010%2E5H18V15H4%2E5v%2D4%2E5zM3%2E75%2018h15A2%2E25%202%2E25%200%200021%2015%2E75v%2D6a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25h%2D15A2%2E25%202%2E25%200%20001%2E5%209%2E75v6A2%2E25%202%2E25%200%20003%2E75%2018z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Battery100Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 10.5h.375c.621 0 1.125.504 1.125 1.125v2.25c0 .621-.504 1.125-1.125 1.125H21M4.5 10.5H18V15H4.5v-4.5zM3.75 18h15A2.25 2.25 0 0021 15.75v-6a2.25 2.25 0 00-2.25-2.25h-15A2.25 2.25 0 001.5 9.75v6A2.25 2.25 0 003.75 18z"/>
</svg>
  }
}
