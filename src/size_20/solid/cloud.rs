use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M1%2012%2E5A4%2E5%204%2E5%200%20005%2E5%2017H15a4%204%200%20001%2E866%2D7%2E539%203%2E504%203%2E504%200%2000%2D4%2E504%2D4%2E272A4%2E5%204%2E5%200%20004%2E06%208%2E235%204%2E502%204%2E502%200%20001%2012%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CloudIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M1 12.5A4.5 4.5 0 005.5 17H15a4 4 0 001.866-7.539 3.504 3.504 0 00-4.504-4.272A4.5 4.5 0 004.06 8.235 4.502 4.502 0 001 12.5z"/>
</svg>
  }
}