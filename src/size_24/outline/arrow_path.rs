use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M16%2E023%209%2E348h4%2E992v%2D%2E001M2%2E985%2019%2E644v%2D4%2E992m0%200h4%2E992m%2D4%2E993%200l3%2E181%203%2E183a8%2E25%208%2E25%200%200013%2E803%2D3%2E7M4%2E031%209%2E865a8%2E25%208%2E25%200%200113%2E803%2D3%2E7l3%2E181%203%2E182m0%2D4%2E991v4%2E99%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowPathIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"/>
</svg>
  }
}
