use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M11%2E412%2015%2E655L9%2E75%2021%2E75l3%2E745%2D4%2E012M9%2E257%2013%2E5H3%2E75l2%2E659%2D2%2E849m2%2E048%2D2%2E194L14%2E25%202%2E25%2012%2010%2E5h8%2E25l%2D4%2E707%205%2E043M8%2E457%208%2E457L3%203m5%2E457%205%2E457l7%2E086%207%2E086m0%200L21%2021%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BoltSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M11.412 15.655L9.75 21.75l3.745-4.012M9.257 13.5H3.75l2.659-2.849m2.048-2.194L14.25 2.25 12 10.5h8.25l-4.707 5.043M8.457 8.457L3 3m5.457 5.457l7.086 7.086m0 0L21 21"/>
</svg>
  }
}