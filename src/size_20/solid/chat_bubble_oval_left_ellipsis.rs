use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%203c%2D4%2E31%200%2D8%203%2E033%2D8%207%200%202%2E024%2E978%203%2E825%202%2E499%205%2E085a3%2E478%203%2E478%200%2001%2D%2E522%201%2E756%2E75%2E75%200%2000%2E584%201%2E143%205%2E976%205%2E976%200%20003%2E936%2D1%2E108c%2E487%2E082%2E99%2E124%201%2E503%2E124%204%2E31%200%208%2D3%2E033%208%2D7s%2D3%2E69%2D7%2D8%2D7zm0%208a1%201%200%20100%2D2%201%201%200%20000%202zm%2D2%2D1a1%201%200%2011%2D2%200%201%201%200%20012%200zm5%201a1%201%200%20100%2D2%201%201%200%20000%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleOvalLeftEllipsisIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 3c-4.31 0-8 3.033-8 7 0 2.024.978 3.825 2.499 5.085a3.478 3.478 0 01-.522 1.756.75.75 0 00.584 1.143 5.976 5.976 0 003.936-1.108c.487.082.99.124 1.503.124 4.31 0 8-3.033 8-7s-3.69-7-8-7zm0 8a1 1 0 100-2 1 1 0 000 2zm-2-1a1 1 0 11-2 0 1 1 0 012 0zm5 1a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
</svg>
  }
}