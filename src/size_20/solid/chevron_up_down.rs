use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%203a%2E75%2E75%200%2001%2E55%2E24l3%2E25%203%2E5a%2E75%2E75%200%2011%2D1%2E1%201%2E02L10%204%2E852%207%2E3%207%2E76a%2E75%2E75%200%2001%2D1%2E1%2D1%2E02l3%2E25%2D3%2E5A%2E75%2E75%200%200110%203zm%2D3%2E76%209%2E2a%2E75%2E75%200%20011%2E06%2E04l2%2E7%202%2E908%202%2E7%2D2%2E908a%2E75%2E75%200%20111%2E1%201%2E02l%2D3%2E25%203%2E5a%2E75%2E75%200%2001%2D1%2E1%200l%2D3%2E25%2D3%2E5a%2E75%2E75%200%2001%2E04%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChevronUpDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd"/>
</svg>
  }
}