use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%203v1%2E5M3%2021v%2D6m0%200l2%2E77%2D%2E693a9%209%200%20016%2E208%2E682l%2E108%2E054a9%209%200%20006%2E086%2E71l3%2E114%2D%2E732a48%2E524%2048%2E524%200%2001%2D%2E005%2D10%2E499l%2D3%2E11%2E732a9%209%200%2001%2D6%2E085%2D%2E711l%2D%2E108%2D%2E054a9%209%200%2000%2D6%2E208%2D%2E682L3%204%2E5M3%2015V4%2E5%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FlagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3 3v1.5M3 21v-6m0 0l2.77-.693a9 9 0 016.208.682l.108.054a9 9 0 006.086.71l3.114-.732a48.524 48.524 0 01-.005-10.499l-3.11.732a9 9 0 01-6.085-.711l-.108-.054a9 9 0 00-6.208-.682L3 4.5M3 15V4.5"/>
</svg>
  }
}