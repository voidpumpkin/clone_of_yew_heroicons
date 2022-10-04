use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2016%2E811c0%20%2E864%2D%2E933%201%2E405%2D1%2E683%2E977l%2D7%2E108%2D4%2E062a1%2E125%201%2E125%200%20010%2D1%2E953l7%2E108%2D4%2E062A1%2E125%201%2E125%200%200121%208%2E688v8%2E123zM11%2E25%2016%2E811c0%20%2E864%2D%2E933%201%2E405%2D1%2E683%2E977l%2D7%2E108%2D4%2E062a1%2E125%201%2E125%200%20010%2D1%2E953L9%2E567%207%2E71a1%2E125%201%2E125%200%20011%2E683%2E977v8%2E123z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BackwardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 16.811c0 .864-.933 1.405-1.683.977l-7.108-4.062a1.125 1.125 0 010-1.953l7.108-4.062A1.125 1.125 0 0121 8.688v8.123zM11.25 16.811c0 .864-.933 1.405-1.683.977l-7.108-4.062a1.125 1.125 0 010-1.953L9.567 7.71a1.125 1.125 0 011.683.977v8.123z"/>
</svg>
  }
}