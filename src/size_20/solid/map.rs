use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M8%2E157%202%2E175a1%2E5%201%2E5%200%2000%2D1%2E147%200l%2D4%2E084%201%2E69A1%2E5%201%2E5%200%20002%205%2E251v10%2E877a1%2E5%201%2E5%200%20002%2E074%201%2E386l3%2E51%2D1%2E453%204%2E26%201%2E763a1%2E5%201%2E5%200%20001%2E146%200l4%2E083%2D1%2E69A1%2E5%201%2E5%200%200018%2014%2E748V3%2E873a1%2E5%201%2E5%200%2000%2D2%2E073%2D1%2E386l%2D3%2E51%201%2E452%2D4%2E26%2D1%2E763zM7%2E58%205a%2E75%2E75%200%2001%2E75%2E75v6%2E5a%2E75%2E75%200%2001%2D1%2E5%200v%2D6%2E5A%2E75%2E75%200%20017%2E58%205zm5%2E59%202%2E75a%2E75%2E75%200%2000%2D1%2E5%200v6%2E5a%2E75%2E75%200%20001%2E5%200v%2D6%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MapIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M8.157 2.175a1.5 1.5 0 00-1.147 0l-4.084 1.69A1.5 1.5 0 002 5.251v10.877a1.5 1.5 0 002.074 1.386l3.51-1.453 4.26 1.763a1.5 1.5 0 001.146 0l4.083-1.69A1.5 1.5 0 0018 14.748V3.873a1.5 1.5 0 00-2.073-1.386l-3.51 1.452-4.26-1.763zM7.58 5a.75.75 0 01.75.75v6.5a.75.75 0 01-1.5 0v-6.5A.75.75 0 017.58 5zm5.59 2.75a.75.75 0 00-1.5 0v6.5a.75.75 0 001.5 0v-6.5z" clip-rule="evenodd"/>
</svg>
  }
}
