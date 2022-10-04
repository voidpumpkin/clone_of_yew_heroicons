use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M14%2E857%2017%2E082a23%2E848%2023%2E848%200%20005%2E454%2D1%2E31A8%2E967%208%2E967%200%200118%209%2E75v%2D%2E7V9A6%206%200%20006%209v%2E75a8%2E967%208%2E967%200%2001%2D2%2E312%206%2E022c1%2E733%2E64%203%2E56%201%2E085%205%2E455%201%2E31m5%2E714%200a24%2E255%2024%2E255%200%2001%2D5%2E714%200m5%2E714%200a3%203%200%2011%2D5%2E714%200%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BellIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0"/>
</svg>
  }
}
