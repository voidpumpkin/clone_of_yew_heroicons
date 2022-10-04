use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E628%201%2E601C5%2E028%201%2E206%207%2E49%201%2010%201s4%2E973%2E206%207%2E372%2E601a%2E75%2E75%200%2001%2E628%2E74v2%2E288a2%2E25%202%2E25%200%2001%2D%2E659%201%2E59l%2D4%2E682%204%2E683a2%2E25%202%2E25%200%2000%2D%2E659%201%2E59v3%2E037c0%20%2E684%2D%2E31%201%2E33%2D%2E844%201%2E757l%2D1%2E937%201%2E55A%2E75%2E75%200%20018%2018%2E25v%2D5%2E757a2%2E25%202%2E25%200%2000%2D%2E659%2D1%2E591L2%2E659%206%2E22A2%2E25%202%2E25%200%20012%204%2E629V2%2E34a%2E75%2E75%200%2001%2E628%2D%2E74z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FunnelIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.628 1.601C5.028 1.206 7.49 1 10 1s4.973.206 7.372.601a.75.75 0 01.628.74v2.288a2.25 2.25 0 01-.659 1.59l-4.682 4.683a2.25 2.25 0 00-.659 1.59v3.037c0 .684-.31 1.33-.844 1.757l-1.937 1.55A.75.75 0 018 18.25v-5.757a2.25 2.25 0 00-.659-1.591L2.659 6.22A2.25 2.25 0 012 4.629V2.34a.75.75 0 01.628-.74z" clip-rule="evenodd"/>
</svg>
  }
}