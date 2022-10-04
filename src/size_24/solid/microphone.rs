use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M8%2E25%204%2E5a3%2E75%203%2E75%200%20117%2E5%200v8%2E25a3%2E75%203%2E75%200%2011%2D7%2E5%200V4%2E5z%22%2F%3E%20%3Cpath%20d%3D%22M6%2010%2E5a%2E75%2E75%200%2001%2E75%2E75v1%2E5a5%2E25%205%2E25%200%201010%2E5%200v%2D1%2E5a%2E75%2E75%200%20011%2E5%200v1%2E5a6%2E751%206%2E751%200%2001%2D6%206%2E709v2%2E291h3a%2E75%2E75%200%20010%201%2E5h%2D7%2E5a%2E75%2E75%200%20010%2D1%2E5h3v%2D2%2E291a6%2E751%206%2E751%200%2001%2D6%2D6%2E709v%2D1%2E5A%2E75%2E75%200%20016%2010%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MicrophoneIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M8.25 4.5a3.75 3.75 0 117.5 0v8.25a3.75 3.75 0 11-7.5 0V4.5z"/>
  <path d="M6 10.5a.75.75 0 01.75.75v1.5a5.25 5.25 0 1010.5 0v-1.5a.75.75 0 011.5 0v1.5a6.751 6.751 0 01-6 6.709v2.291h3a.75.75 0 010 1.5h-7.5a.75.75 0 010-1.5h3v-2.291a6.751 6.751 0 01-6-6.709v-1.5A.75.75 0 016 10.5z"/>
</svg>
  }
}
