use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%208%2E25V18a2%2E25%202%2E25%200%20002%2E25%202%2E25h13%2E5A2%2E25%202%2E25%200%200021%2018V8%2E25m%2D18%200V6a2%2E25%202%2E25%200%20012%2E25%2D2%2E25h13%2E5A2%2E25%202%2E25%200%200121%206v2%2E25m%2D18%200h18M5%2E25%206h%2E008v%2E008H5%2E25V6zM7%2E5%206h%2E008v%2E008H7%2E5V6zm2%2E25%200h%2E008v%2E008H9%2E75V6z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn WindowIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3 8.25V18a2.25 2.25 0 002.25 2.25h13.5A2.25 2.25 0 0021 18V8.25m-18 0V6a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 6v2.25m-18 0h18M5.25 6h.008v.008H5.25V6zM7.5 6h.008v.008H7.5V6zm2.25 0h.008v.008H9.75V6z"/>
</svg>
  }
}