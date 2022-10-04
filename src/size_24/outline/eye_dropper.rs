use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2011%2E25l1%2E5%201%2E5%2E75%2D%2E75V8%2E758l2%2E276%2D%2E61a3%203%200%2010%2D3%2E675%2D3%2E675l%2D%2E61%202%2E277H12l%2D%2E75%2E75%201%2E5%201%2E5M15%2011%2E25l%2D8%2E47%208%2E47c%2D%2E34%2E34%2D%2E8%2E53%2D1%2E28%2E53s%2D%2E94%2E19%2D1%2E28%2E53l%2D%2E97%2E97%2D%2E75%2D%2E75%2E97%2D%2E97c%2E34%2D%2E34%2E53%2D%2E8%2E53%2D1%2E28s%2E19%2D%2E94%2E53%2D1%2E28L12%2E75%209M15%2011%2E25L12%2E75%209%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EyeDropperIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l1.5 1.5.75-.75V8.758l2.276-.61a3 3 0 10-3.675-3.675l-.61 2.277H12l-.75.75 1.5 1.5M15 11.25l-8.47 8.47c-.34.34-.8.53-1.28.53s-.94.19-1.28.53l-.97.97-.75-.75.97-.97c.34-.34.53-.8.53-1.28s.19-.94.53-1.28L12.75 9M15 11.25L12.75 9"/>
</svg>
  }
}