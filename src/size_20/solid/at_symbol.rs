use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E404%2014%2E596A6%2E5%206%2E5%200%201116%2E5%2010a1%2E25%201%2E25%200%2001%2D2%2E5%200%204%204%200%2010%2D%2E571%202%2E06A2%2E75%202%2E75%200%200018%2010a8%208%200%2010%2D2%2E343%205%2E657%2E75%2E75%200%2000%2D1%2E06%2D1%2E06%206%2E5%206%2E5%200%2001%2D9%2E193%200zM10%207%2E5a2%2E5%202%2E5%200%20100%205%202%2E5%202%2E5%200%20000%2D5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn AtSymbolIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.404 14.596A6.5 6.5 0 1116.5 10a1.25 1.25 0 01-2.5 0 4 4 0 10-.571 2.06A2.75 2.75 0 0018 10a8 8 0 10-2.343 5.657.75.75 0 00-1.06-1.06 6.5 6.5 0 01-9.193 0zM10 7.5a2.5 2.5 0 100 5 2.5 2.5 0 000-5z" clip-rule="evenodd"/>
</svg>
  }
}