use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M13%2E75%207h%2D3v5%2E296l1%2E943%2D2%2E048a%2E75%2E75%200%20011%2E114%201%2E004l%2D3%2E25%203%2E5a%2E75%2E75%200%2001%2D1%2E114%200l%2D3%2E25%2D3%2E5a%2E75%2E75%200%20111%2E114%2D1%2E004l1%2E943%202%2E048V7h1%2E5V1%2E75a%2E75%2E75%200%2000%2D1%2E5%200V7h%2D3A2%2E25%202%2E25%200%20004%209%2E25v7%2E5A2%2E25%202%2E25%200%20006%2E25%2019h7%2E5A2%2E25%202%2E25%200%200016%2016%2E75v%2D7%2E5A2%2E25%202%2E25%200%200013%2E75%207z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownOnSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M13.75 7h-3v5.296l1.943-2.048a.75.75 0 011.114 1.004l-3.25 3.5a.75.75 0 01-1.114 0l-3.25-3.5a.75.75 0 111.114-1.004l1.943 2.048V7h1.5V1.75a.75.75 0 00-1.5 0V7h-3A2.25 2.25 0 004 9.25v7.5A2.25 2.25 0 006.25 19h7.5A2.25 2.25 0 0016 16.75v-7.5A2.25 2.25 0 0013.75 7z"/>
</svg>
  }
}
