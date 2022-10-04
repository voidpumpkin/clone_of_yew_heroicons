use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E25%202%2E25a3%203%200%2000%2D3%203v4%2E318a3%203%200%2000%2E879%202%2E121l9%2E58%209%2E581c%2E92%2E92%202%2E39%201%2E186%203%2E548%2E428a18%2E849%2018%2E849%200%20005%2E441%2D5%2E44c%2E758%2D1%2E16%2E492%2D2%2E629%2D%2E428%2D3%2E548l%2D9%2E58%2D9%2E581a3%203%200%2000%2D2%2E122%2D%2E879H5%2E25zM6%2E375%207%2E5a1%2E125%201%2E125%200%20100%2D2%2E25%201%2E125%201%2E125%200%20000%202%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.25 2.25a3 3 0 00-3 3v4.318a3 3 0 00.879 2.121l9.58 9.581c.92.92 2.39 1.186 3.548.428a18.849 18.849 0 005.441-5.44c.758-1.16.492-2.629-.428-3.548l-9.58-9.581a3 3 0 00-2.122-.879H5.25zM6.375 7.5a1.125 1.125 0 100-2.25 1.125 1.125 0 000 2.25z" clip-rule="evenodd"/>
</svg>
  }
}
