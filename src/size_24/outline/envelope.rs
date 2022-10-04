use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2E75%206%2E75v10%2E5a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25h%2D15a2%2E25%202%2E25%200%2001%2D2%2E25%2D2%2E25V6%2E75m19%2E5%200A2%2E25%202%2E25%200%200019%2E5%204%2E5h%2D15a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25m19%2E5%200v%2E243a2%2E25%202%2E25%200%2001%2D1%2E07%201%2E916l%2D7%2E5%204%2E615a2%2E25%202%2E25%200%2001%2D2%2E36%200L3%2E32%208%2E91a2%2E25%202%2E25%200%2001%2D1%2E07%2D1%2E916V6%2E75%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EnvelopeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75"/>
</svg>
  }
}
