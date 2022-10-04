use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%208%2E688c0%2D%2E864%2E933%2D1%2E405%201%2E683%2D%2E977l7%2E108%204%2E062a1%2E125%201%2E125%200%20010%201%2E953l%2D7%2E108%204%2E062A1%2E125%201%2E125%200%20013%2016%2E81V8%2E688zM12%2E75%208%2E688c0%2D%2E864%2E933%2D1%2E405%201%2E683%2D%2E977l7%2E108%204%2E062a1%2E125%201%2E125%200%20010%201%2E953l%2D7%2E108%204%2E062a1%2E125%201%2E125%200%2001%2D1%2E683%2D%2E977V8%2E688z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ForwardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3 8.688c0-.864.933-1.405 1.683-.977l7.108 4.062a1.125 1.125 0 010 1.953l-7.108 4.062A1.125 1.125 0 013 16.81V8.688zM12.75 8.688c0-.864.933-1.405 1.683-.977l7.108 4.062a1.125 1.125 0 010 1.953l-7.108 4.062a1.125 1.125 0 01-1.683-.977V8.688z"/>
</svg>
  }
}