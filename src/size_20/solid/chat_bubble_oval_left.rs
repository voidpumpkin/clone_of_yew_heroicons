use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2010c0%2D3%2E967%203%2E69%2D7%208%2D7%204%2E31%200%208%203%2E033%208%207s%2D3%2E69%207%2D8%207a9%2E165%209%2E165%200%2001%2D1%2E504%2D%2E123%205%2E976%205%2E976%200%2001%2D3%2E935%201%2E107%2E75%2E75%200%2001%2D%2E584%2D1%2E143%203%2E478%203%2E478%200%2000%2E522%2D1%2E756C2%2E979%2013%2E825%202%2012%2E025%202%2010z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleOvalLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2 10c0-3.967 3.69-7 8-7 4.31 0 8 3.033 8 7s-3.69 7-8 7a9.165 9.165 0 01-1.504-.123 5.976 5.976 0 01-3.935 1.107.75.75 0 01-.584-1.143 3.478 3.478 0 00.522-1.756C2.979 13.825 2 12.025 2 10z" clip-rule="evenodd"/>
</svg>
  }
}
