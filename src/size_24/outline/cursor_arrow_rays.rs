use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2E042%2021%2E672L13%2E684%2016%2E6m0%200l%2D2%2E51%202%2E225%2E569%2D9%2E47%205%2E227%207%2E917%2D3%2E286%2D%2E672zM12%202%2E25V4%2E5m5%2E834%2E166l%2D1%2E591%201%2E591M20%2E25%2010%2E5H18M7%2E757%2014%2E743l%2D1%2E59%201%2E59M6%2010%2E5H3%2E75m4%2E007%2D4%2E243l%2D1%2E59%2D1%2E59%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CursorArrowRaysIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.042 21.672L13.684 16.6m0 0l-2.51 2.225.569-9.47 5.227 7.917-3.286-.672zM12 2.25V4.5m5.834.166l-1.591 1.591M20.25 10.5H18M7.757 14.743l-1.59 1.59M6 10.5H3.75m4.007-4.243l-1.59-1.59"/>
</svg>
  }
}