use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%2E28%205%2E22a%2E75%2E75%200%20010%201%2E06L2%2E56%2010l3%2E72%203%2E72a%2E75%2E75%200%2001%2D1%2E06%201%2E06L%2E97%2010%2E53a%2E75%2E75%200%20010%2D1%2E06l4%2E25%2D4%2E25a%2E75%2E75%200%20011%2E06%200zm7%2E44%200a%2E75%2E75%200%20011%2E06%200l4%2E25%204%2E25a%2E75%2E75%200%20010%201%2E06l%2D4%2E25%204%2E25a%2E75%2E75%200%2001%2D1%2E06%2D1%2E06L17%2E44%2010l%2D3%2E72%2D3%2E72a%2E75%2E75%200%20010%2D1%2E06zM11%2E377%202%2E011a%2E75%2E75%200%2001%2E612%2E867l%2D2%2E5%2014%2E5a%2E75%2E75%200%2001%2D1%2E478%2D%2E255l2%2E5%2D14%2E5a%2E75%2E75%200%2001%2E866%2D%2E612z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CodeBracketIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6.28 5.22a.75.75 0 010 1.06L2.56 10l3.72 3.72a.75.75 0 01-1.06 1.06L.97 10.53a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 0zm7.44 0a.75.75 0 011.06 0l4.25 4.25a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06-1.06L17.44 10l-3.72-3.72a.75.75 0 010-1.06zM11.377 2.011a.75.75 0 01.612.867l-2.5 14.5a.75.75 0 01-1.478-.255l2.5-14.5a.75.75 0 01.866-.612z" clip-rule="evenodd"/>
</svg>
  }
}
