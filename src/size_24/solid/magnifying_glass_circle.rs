use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M8%2E25%2010%2E875a2%2E625%202%2E625%200%20115%2E25%200%202%2E625%202%2E625%200%2001%2D5%2E25%200z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%202%2E25c%2D5%2E385%200%2D9%2E75%204%2E365%2D9%2E75%209%2E75s4%2E365%209%2E75%209%2E75%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75S17%2E385%202%2E25%2012%202%2E25zm%2D1%2E125%204%2E5a4%2E125%204%2E125%200%20102%2E338%207%2E524l2%2E007%202%2E006a%2E75%2E75%200%20101%2E06%2D1%2E06l%2D2%2E006%2D2%2E007a4%2E125%204%2E125%200%2000%2D3%2E399%2D6%2E463z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MagnifyingGlassCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M8.25 10.875a2.625 2.625 0 115.25 0 2.625 2.625 0 01-5.25 0z"/>
  <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-1.125 4.5a4.125 4.125 0 102.338 7.524l2.007 2.006a.75.75 0 101.06-1.06l-2.006-2.007a4.125 4.125 0 00-3.399-6.463z" clip-rule="evenodd"/>
</svg>
  }
}