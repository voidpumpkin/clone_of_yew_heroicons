use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%2E375%202%2E25a4%2E125%204%2E125%200%20100%208%2E25%204%2E125%204%2E125%200%20000%2D8%2E25zM10%2E375%2012a7%2E125%207%2E125%200%2000%2D7%2E124%207%2E247%2E75%2E75%200%2000%2E363%2E63%2013%2E067%2013%2E067%200%20006%2E761%201%2E873c2%2E472%200%204%2E786%2D%2E684%206%2E76%2D1%2E873a%2E75%2E75%200%2000%2E364%2D%2E63l%2E001%2D%2E12v%2D%2E002A7%2E125%207%2E125%200%200010%2E375%2012zM16%209%2E75a%2E75%2E75%200%20000%201%2E5h6a%2E75%2E75%200%20000%2D1%2E5h%2D6z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserMinusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M10.375 2.25a4.125 4.125 0 100 8.25 4.125 4.125 0 000-8.25zM10.375 12a7.125 7.125 0 00-7.124 7.247.75.75 0 00.363.63 13.067 13.067 0 006.761 1.873c2.472 0 4.786-.684 6.76-1.873a.75.75 0 00.364-.63l.001-.12v-.002A7.125 7.125 0 0010.375 12zM16 9.75a.75.75 0 000 1.5h6a.75.75 0 000-1.5h-6z"/>
</svg>
  }
}
