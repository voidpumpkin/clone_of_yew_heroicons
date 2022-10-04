use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M7%204a3%203%200%20016%200v6a3%203%200%2011%2D6%200V4z%22%2F%3E%20%3Cpath%20d%3D%22M5%2E5%209%2E643a%2E75%2E75%200%2000%2D1%2E5%200V10c0%203%2E06%202%2E29%205%2E585%205%2E25%205%2E954V17%2E5h%2D1%2E5a%2E75%2E75%200%20000%201%2E5h4%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D1%2E5v%2D1%2E546A6%2E001%206%2E001%200%200016%2010v%2D%2E357a%2E75%2E75%200%2000%2D1%2E5%200V10a4%2E5%204%2E5%200%2001%2D9%200v%2D%2E357z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MicrophoneIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M7 4a3 3 0 016 0v6a3 3 0 11-6 0V4z"/>
  <path d="M5.5 9.643a.75.75 0 00-1.5 0V10c0 3.06 2.29 5.585 5.25 5.954V17.5h-1.5a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-1.5v-1.546A6.001 6.001 0 0016 10v-.357a.75.75 0 00-1.5 0V10a4.5 4.5 0 01-9 0v-.357z"/>
</svg>
  }
}