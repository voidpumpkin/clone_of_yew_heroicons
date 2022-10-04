use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M6%2E5%209a2%2E5%202%2E5%200%20115%200%202%2E5%202%2E5%200%2001%2D5%200z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2018a8%208%200%20100%2D16%208%208%200%20000%2016zM9%205a4%204%200%20102%2E248%207%2E309l1%2E472%201%2E471a%2E75%2E75%200%20101%2E06%2D1%2E06l%2D1%2E471%2D1%2E472A4%204%200%20009%205z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MagnifyingGlassCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M6.5 9a2.5 2.5 0 115 0 2.5 2.5 0 01-5 0z"/>
  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9 5a4 4 0 102.248 7.309l1.472 1.471a.75.75 0 101.06-1.06l-1.471-1.472A4 4 0 009 5z" clip-rule="evenodd"/>
</svg>
  }
}