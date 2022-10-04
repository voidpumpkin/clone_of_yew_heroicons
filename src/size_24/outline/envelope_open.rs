use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2E75%209v%2E906a2%2E25%202%2E25%200%2001%2D1%2E183%201%2E981l%2D6%2E478%203%2E488M2%2E25%209v%2E906a2%2E25%202%2E25%200%20001%2E183%201%2E981l6%2E478%203%2E488m8%2E839%202%2E51l%2D4%2E66%2D2%2E51m0%200l%2D1%2E023%2D%2E55a2%2E25%202%2E25%200%2000%2D2%2E134%200l%2D1%2E022%2E55m0%200l%2D4%2E661%202%2E51m16%2E5%201%2E615a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25h%2D15a2%2E25%202%2E25%200%2001%2D2%2E25%2D2%2E25V8%2E844a2%2E25%202%2E25%200%20011%2E183%2D1%2E98l7%2E5%2D4%2E04a2%2E25%202%2E25%200%20012%2E134%200l7%2E5%204%2E04a2%2E25%202%2E25%200%20011%2E183%201%2E98V19%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EnvelopeOpenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 9v.906a2.25 2.25 0 01-1.183 1.981l-6.478 3.488M2.25 9v.906a2.25 2.25 0 001.183 1.981l6.478 3.488m8.839 2.51l-4.66-2.51m0 0l-1.023-.55a2.25 2.25 0 00-2.134 0l-1.022.55m0 0l-4.661 2.51m16.5 1.615a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V8.844a2.25 2.25 0 011.183-1.98l7.5-4.04a2.25 2.25 0 012.134 0l7.5 4.04a2.25 2.25 0 011.183 1.98V19.5z"/>
</svg>
  }
}
