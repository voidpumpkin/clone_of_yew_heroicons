use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%2E97%202%2E47a%2E75%2E75%200%20011%2E06%200l4%2E5%204%2E5a%2E75%2E75%200%2001%2D1%2E06%201%2E06L8%2E25%204%2E81V16%2E5a%2E75%2E75%200%2001%2D1%2E5%200V4%2E81L3%2E53%208%2E03a%2E75%2E75%200%2001%2D1%2E06%2D1%2E06l4%2E5%2D4%2E5zm9%2E53%204%2E28a%2E75%2E75%200%2001%2E75%2E75v11%2E69l3%2E22%2D3%2E22a%2E75%2E75%200%20111%2E06%201%2E06l%2D4%2E5%204%2E5a%2E75%2E75%200%2001%2D1%2E06%200l%2D4%2E5%2D4%2E5a%2E75%2E75%200%20111%2E06%2D1%2E06l3%2E22%203%2E22V7%2E5a%2E75%2E75%200%2001%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowsUpDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6.97 2.47a.75.75 0 011.06 0l4.5 4.5a.75.75 0 01-1.06 1.06L8.25 4.81V16.5a.75.75 0 01-1.5 0V4.81L3.53 8.03a.75.75 0 01-1.06-1.06l4.5-4.5zm9.53 4.28a.75.75 0 01.75.75v11.69l3.22-3.22a.75.75 0 111.06 1.06l-4.5 4.5a.75.75 0 01-1.06 0l-4.5-4.5a.75.75 0 111.06-1.06l3.22 3.22V7.5a.75.75 0 01.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}
