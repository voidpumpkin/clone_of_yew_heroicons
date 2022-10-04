use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%2021v%2D8%2E25M15%2E75%2021v%2D8%2E25M8%2E25%2021v%2D8%2E25M3%209l9%2D6%209%206m%2D1%2E5%2012V10%2E332A48%2E36%2048%2E36%200%200012%209%2E75c%2D2%2E551%200%2D5%2E056%2E2%2D7%2E5%2E582V21M3%2021h18M12%206%2E75h%2E008v%2E008H12V6%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BuildingLibraryIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 21v-8.25M15.75 21v-8.25M8.25 21v-8.25M3 9l9-6 9 6m-1.5 12V10.332A48.36 48.36 0 0012 9.75c-2.551 0-5.056.2-7.5.582V21M3 21h18M12 6.75h.008v.008H12V6.75z"/>
</svg>
  }
}
