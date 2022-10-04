use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M14%2E447%203%2E027a%2E75%2E75%200%2001%2E527%2E92l%2D4%2E5%2016%2E5a%2E75%2E75%200%2001%2D1%2E448%2D%2E394l4%2E5%2D16%2E5a%2E75%2E75%200%2001%2E921%2D%2E526zM16%2E72%206%2E22a%2E75%2E75%200%20011%2E06%200l5%2E25%205%2E25a%2E75%2E75%200%20010%201%2E06l%2D5%2E25%205%2E25a%2E75%2E75%200%2011%2D1%2E06%2D1%2E06L21%2E44%2012l%2D4%2E72%2D4%2E72a%2E75%2E75%200%20010%2D1%2E06zm%2D9%2E44%200a%2E75%2E75%200%20010%201%2E06L2%2E56%2012l4%2E72%204%2E72a%2E75%2E75%200%2011%2D1%2E06%201%2E06L%2E97%2012%2E53a%2E75%2E75%200%20010%2D1%2E06l5%2E25%2D5%2E25a%2E75%2E75%200%20011%2E06%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CodeBracketIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M14.447 3.027a.75.75 0 01.527.92l-4.5 16.5a.75.75 0 01-1.448-.394l4.5-16.5a.75.75 0 01.921-.526zM16.72 6.22a.75.75 0 011.06 0l5.25 5.25a.75.75 0 010 1.06l-5.25 5.25a.75.75 0 11-1.06-1.06L21.44 12l-4.72-4.72a.75.75 0 010-1.06zm-9.44 0a.75.75 0 010 1.06L2.56 12l4.72 4.72a.75.75 0 11-1.06 1.06L.97 12.53a.75.75 0 010-1.06l5.25-5.25a.75.75 0 011.06 0z" clip-rule="evenodd"/>
</svg>
  }
}