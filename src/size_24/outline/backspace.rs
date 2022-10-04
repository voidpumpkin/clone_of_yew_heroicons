use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%209%2E75L14%2E25%2012m0%200l2%2E25%202%2E25M14%2E25%2012l2%2E25%2D2%2E25M14%2E25%2012L12%2014%2E25m%2D2%2E58%204%2E92l%2D6%2E375%2D6%2E375a1%2E125%201%2E125%200%20010%2D1%2E59L9%2E42%204%2E83c%2E211%2D%2E211%2E498%2D%2E33%2E796%2D%2E33H19%2E5a2%2E25%202%2E25%200%20012%2E25%202%2E25v10%2E5a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25h%2D9%2E284c%2D%2E298%200%2D%2E585%2D%2E119%2D%2E796%2D%2E33z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BackspaceIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75L14.25 12m0 0l2.25 2.25M14.25 12l2.25-2.25M14.25 12L12 14.25m-2.58 4.92l-6.375-6.375a1.125 1.125 0 010-1.59L9.42 4.83c.211-.211.498-.33.796-.33H19.5a2.25 2.25 0 012.25 2.25v10.5a2.25 2.25 0 01-2.25 2.25h-9.284c-.298 0-.585-.119-.796-.33z"/>
</svg>
  }
}