use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2E75%206h%2D2v4%2E25a%2E75%2E75%200%2001%2D1%2E5%200V6h1%2E5V3%2E704l%2E943%201%2E048a%2E75%2E75%200%20001%2E114%2D1%2E004l%2D2%2E25%2D2%2E5a%2E75%2E75%200%2000%2D1%2E114%200l%2D2%2E25%202%2E5a%2E75%2E75%200%20001%2E114%201%2E004l%2E943%2D1%2E048V6h%2D2A2%2E25%202%2E25%200%20003%208%2E25v4%2E5A2%2E25%202%2E25%200%20005%2E25%2015h5%2E5A2%2E25%202%2E25%200%200013%2012%2E75v%2D4%2E5A2%2E25%202%2E25%200%200010%2E75%206zM7%2016%2E75v%2D%2E25h3%2E75a3%2E75%203%2E75%200%20003%2E75%2D3%2E75V10h%2E25A2%2E25%202%2E25%200%200117%2012%2E25v4%2E5A2%2E25%202%2E25%200%200114%2E75%2019h%2D5%2E5A2%2E25%202%2E25%200%20017%2016%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUpOnSquareStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10.75 6h-2v4.25a.75.75 0 01-1.5 0V6h1.5V3.704l.943 1.048a.75.75 0 001.114-1.004l-2.25-2.5a.75.75 0 00-1.114 0l-2.25 2.5a.75.75 0 001.114 1.004l.943-1.048V6h-2A2.25 2.25 0 003 8.25v4.5A2.25 2.25 0 005.25 15h5.5A2.25 2.25 0 0013 12.75v-4.5A2.25 2.25 0 0010.75 6zM7 16.75v-.25h3.75a3.75 3.75 0 003.75-3.75V10h.25A2.25 2.25 0 0117 12.25v4.5A2.25 2.25 0 0114.75 19h-5.5A2.25 2.25 0 017 16.75z" clip-rule="evenodd"/>
</svg>
  }
}
