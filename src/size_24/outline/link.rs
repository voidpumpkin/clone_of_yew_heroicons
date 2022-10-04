use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M13%2E19%208%2E688a4%2E5%204%2E5%200%20011%2E242%207%2E244l%2D4%2E5%204%2E5a4%2E5%204%2E5%200%2001%2D6%2E364%2D6%2E364l1%2E757%2D1%2E757m13%2E35%2D%2E622l1%2E757%2D1%2E757a4%2E5%204%2E5%200%2000%2D6%2E364%2D6%2E364l%2D4%2E5%204%2E5a4%2E5%204%2E5%200%20001%2E242%207%2E244%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LinkIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244"/>
</svg>
  }
}
