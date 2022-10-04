use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E5%202%2E75a%2E75%2E75%200%2000%2D1%2E5%200v14%2E5a%2E75%2E75%200%20001%2E5%200v%2D4%2E392l1%2E657%2D%2E348a6%2E449%206%2E449%200%20014%2E271%2E572%207%2E948%207%2E948%200%20005%2E965%2E524l2%2E078%2D%2E64A%2E75%2E75%200%200018%2012%2E25v%2D8%2E5a%2E75%2E75%200%2000%2D%2E904%2D%2E734l%2D2%2E38%2E501a7%2E25%207%2E25%200%2001%2D4%2E186%2D%2E363l%2D%2E502%2D%2E2a8%2E75%208%2E75%200%2000%2D5%2E053%2D%2E439l%2D1%2E475%2E31V2%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FlagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M3.5 2.75a.75.75 0 00-1.5 0v14.5a.75.75 0 001.5 0v-4.392l1.657-.348a6.449 6.449 0 014.271.572 7.948 7.948 0 005.965.524l2.078-.64A.75.75 0 0018 12.25v-8.5a.75.75 0 00-.904-.734l-2.38.501a7.25 7.25 0 01-4.186-.363l-.502-.2a8.75 8.75 0 00-5.053-.439l-1.475.31V2.75z"/>
</svg>
  }
}