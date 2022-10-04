use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2E879%207%2E519c1%2E171%2D1%2E025%203%2E071%2D1%2E025%204%2E242%200%201%2E172%201%2E025%201%2E172%202%2E687%200%203%2E712%2D%2E203%2E179%2D%2E43%2E326%2D%2E67%2E442%2D%2E745%2E361%2D1%2E45%2E999%2D1%2E45%201%2E827v%2E75M21%2012a9%209%200%2011%2D18%200%209%209%200%200118%200zm%2D9%205%2E25h%2E008v%2E008H12v%2D%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn QuestionMarkCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z"/>
</svg>
  }
}