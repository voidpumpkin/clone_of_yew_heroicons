use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M10%2E5%2021l5%2E25%2D11%2E25L21%2021m%2D9%2D3h7%2E5M3%205%2E621a48%2E474%2048%2E474%200%20016%2D%2E371m0%200c1%2E12%200%202%2E233%2E038%203%2E334%2E114M9%205%2E25V3m3%2E334%202%2E364C11%2E176%2010%2E658%207%2E69%2015%2E08%203%2017%2E502m9%2E334%2D12%2E138c%2E896%2E061%201%2E785%2E147%202%2E666%2E257m%2D4%2E589%208%2E495a18%2E023%2018%2E023%200%2001%2D3%2E827%2D5%2E802%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LanguageIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 21l5.25-11.25L21 21m-9-3h7.5M3 5.621a48.474 48.474 0 016-.371m0 0c1.12 0 2.233.038 3.334.114M9 5.25V3m3.334 2.364C11.176 10.658 7.69 15.08 3 17.502m9.334-12.138c.896.061 1.785.147 2.666.257m-4.589 8.495a18.023 18.023 0 01-3.827-5.802"/>
</svg>
  }
}