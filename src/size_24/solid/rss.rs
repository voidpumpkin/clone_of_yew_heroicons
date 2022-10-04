use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E75%204%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75c8%2E284%200%2015%206%2E716%2015%2015v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75C18%2011%2E708%2012%2E292%206%205%2E25%206H4%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75V4%2E5zm0%206%2E75a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75a8%2E25%208%2E25%200%20018%2E25%208%2E25v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75H12a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75a6%206%200%2000%2D6%2D6H4%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75zm0%207%2E5a1%2E5%201%2E5%200%20113%200%201%2E5%201%2E5%200%2001%2D3%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RssIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.75 4.5a.75.75 0 01.75-.75h.75c8.284 0 15 6.716 15 15v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75C18 11.708 12.292 6 5.25 6H4.5a.75.75 0 01-.75-.75V4.5zm0 6.75a.75.75 0 01.75-.75h.75a8.25 8.25 0 018.25 8.25v.75a.75.75 0 01-.75.75H12a.75.75 0 01-.75-.75v-.75a6 6 0 00-6-6H4.5a.75.75 0 01-.75-.75v-.75zm0 7.5a1.5 1.5 0 113 0 1.5 1.5 0 01-3 0z" clip-rule="evenodd"/>
</svg>
  }
}
