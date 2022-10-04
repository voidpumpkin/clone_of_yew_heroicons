use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%202%2E25c%2D5%2E385%200%2D9%2E75%204%2E365%2D9%2E75%209%2E75s4%2E365%209%2E75%209%2E75%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75S17%2E385%202%2E25%2012%202%2E25zm%2D4%2E28%209%2E22a%2E75%2E75%200%20000%201%2E06l3%203a%2E75%2E75%200%20101%2E06%2D1%2E06l%2D1%2E72%2D1%2E72h5%2E69a%2E75%2E75%200%20000%2D1%2E5h%2D5%2E69l1%2E72%2D1%2E72a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06l%2D3%203z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowLeftCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-4.28 9.22a.75.75 0 000 1.06l3 3a.75.75 0 101.06-1.06l-1.72-1.72h5.69a.75.75 0 000-1.5h-5.69l1.72-1.72a.75.75 0 00-1.06-1.06l-3 3z" clip-rule="evenodd"/>
</svg>
  }
}
