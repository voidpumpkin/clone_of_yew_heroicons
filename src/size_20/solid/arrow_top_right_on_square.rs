use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%2E25%205%2E5a%2E75%2E75%200%2000%2D%2E75%2E75v8%2E5c0%20%2E414%2E336%2E75%2E75%2E75h8%2E5a%2E75%2E75%200%2000%2E75%2D%2E75v%2D4a%2E75%2E75%200%20011%2E5%200v4A2%2E25%202%2E25%200%200112%2E75%2017h%2D8%2E5A2%2E25%202%2E25%200%20012%2014%2E75v%2D8%2E5A2%2E25%202%2E25%200%20014%2E25%204h5a%2E75%2E75%200%20010%201%2E5h%2D5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%2E194%2012%2E753a%2E75%2E75%200%20001%2E06%2E053L16%2E5%204%2E44v2%2E81a%2E75%2E75%200%20001%2E5%200v%2D4%2E5a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D4%2E5a%2E75%2E75%200%20000%201%2E5h2%2E553l%2D9%2E056%208%2E194a%2E75%2E75%200%2000%2D%2E053%201%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowTopRightOnSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M4.25 5.5a.75.75 0 00-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 00.75-.75v-4a.75.75 0 011.5 0v4A2.25 2.25 0 0112.75 17h-8.5A2.25 2.25 0 012 14.75v-8.5A2.25 2.25 0 014.25 4h5a.75.75 0 010 1.5h-5z" clip-rule="evenodd"/>
  <path fill-rule="evenodd" d="M6.194 12.753a.75.75 0 001.06.053L16.5 4.44v2.81a.75.75 0 001.5 0v-4.5a.75.75 0 00-.75-.75h-4.5a.75.75 0 000 1.5h2.553l-9.056 8.194a.75.75 0 00-.053 1.06z" clip-rule="evenodd"/>
</svg>
  }
}
