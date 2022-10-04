use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2E362%205%2E214A8%2E252%208%2E252%200%200112%2021%208%2E25%208%2E25%200%20016%2E038%207%2E048%208%2E287%208%2E287%200%20009%209%2E6a8%2E983%208%2E983%200%20013%2E361%2D6%2E867%208%2E21%208%2E21%200%20003%202%2E48z%22%2F%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%2018a3%2E75%203%2E75%200%2000%2E495%2D7%2E467%205%2E99%205%2E99%200%2000%2D1%2E925%203%2E546%205%2E974%205%2E974%200%2001%2D2%2E133%2D1A3%2E75%203%2E75%200%200012%2018z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FireIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.362 5.214A8.252 8.252 0 0112 21 8.25 8.25 0 016.038 7.048 8.287 8.287 0 009 9.6a8.983 8.983 0 013.361-6.867 8.21 8.21 0 003 2.48z"/>
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 18a3.75 3.75 0 00.495-7.467 5.99 5.99 0 00-1.925 3.546 5.974 5.974 0 01-2.133-1A3.75 3.75 0 0012 18z"/>
</svg>
  }
}