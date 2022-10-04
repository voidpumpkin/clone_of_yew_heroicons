use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E5%203A1%2E5%201%2E5%200%20001%204%2E5v4A1%2E5%201%2E5%200%20002%2E5%2010h6A1%2E5%201%2E5%200%200010%208%2E5v%2D4A1%2E5%201%2E5%200%20008%2E5%203h%2D6zm11%202A1%2E5%201%2E5%200%200012%206%2E5v7a1%2E5%201%2E5%200%20001%2E5%201%2E5h4a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D7A1%2E5%201%2E5%200%200017%2E5%205h%2D4zm%2D10%207A1%2E5%201%2E5%200%20002%2013%2E5v2A1%2E5%201%2E5%200%20003%2E5%2017h6a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D2A1%2E5%201%2E5%200%20009%2E5%2012h%2D6z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RectangleGroupIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.5 3A1.5 1.5 0 001 4.5v4A1.5 1.5 0 002.5 10h6A1.5 1.5 0 0010 8.5v-4A1.5 1.5 0 008.5 3h-6zm11 2A1.5 1.5 0 0012 6.5v7a1.5 1.5 0 001.5 1.5h4a1.5 1.5 0 001.5-1.5v-7A1.5 1.5 0 0017.5 5h-4zm-10 7A1.5 1.5 0 002 13.5v2A1.5 1.5 0 003.5 17h6a1.5 1.5 0 001.5-1.5v-2A1.5 1.5 0 009.5 12h-6z" clip-rule="evenodd"/>
</svg>
  }
}
