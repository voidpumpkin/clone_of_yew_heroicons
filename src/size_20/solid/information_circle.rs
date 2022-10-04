use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M19%2010%2E5a8%2E5%208%2E5%200%2011%2D17%200%208%2E5%208%2E5%200%200117%200zM8%2E25%209%2E75A%2E75%2E75%200%20019%209h%2E253a1%2E75%201%2E75%200%20011%2E709%202%2E13l%2D%2E46%202%2E066a%2E25%2E25%200%2000%2E245%2E304H11a%2E75%2E75%200%20010%201%2E5h%2D%2E253a1%2E75%201%2E75%200%2001%2D1%2E709%2D2%2E13l%2E46%2D2%2E066a%2E25%2E25%200%2000%2D%2E245%2D%2E304H9a%2E75%2E75%200%2001%2D%2E75%2D%2E75zM10%207a1%201%200%20100%2D2%201%201%200%20000%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn InformationCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M19 10.5a8.5 8.5 0 11-17 0 8.5 8.5 0 0117 0zM8.25 9.75A.75.75 0 019 9h.253a1.75 1.75 0 011.709 2.13l-.46 2.066a.25.25 0 00.245.304H11a.75.75 0 010 1.5h-.253a1.75 1.75 0 01-1.709-2.13l.46-2.066a.25.25 0 00-.245-.304H9a.75.75 0 01-.75-.75zM10 7a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
</svg>
  }
}