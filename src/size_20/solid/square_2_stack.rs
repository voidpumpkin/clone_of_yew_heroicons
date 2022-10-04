use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M2%204%2E25A2%2E25%202%2E25%200%20014%2E25%202h6%2E5A2%2E25%202%2E25%200%200113%204%2E25V5%2E5H9%2E25A3%2E75%203%2E75%200%20005%2E5%209%2E25V13H4%2E25A2%2E25%202%2E25%200%20012%2010%2E75v%2D6%2E5z%22%2F%3E%20%3Cpath%20d%3D%22M9%2E25%207A2%2E25%202%2E25%200%20007%209%2E25v6%2E5A2%2E25%202%2E25%200%20009%2E25%2018h6%2E5A2%2E25%202%2E25%200%200018%2015%2E75v%2D6%2E5A2%2E25%202%2E25%200%200015%2E75%207h%2D6%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Square2StackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M2 4.25A2.25 2.25 0 014.25 2h6.5A2.25 2.25 0 0113 4.25V5.5H9.25A3.75 3.75 0 005.5 9.25V13H4.25A2.25 2.25 0 012 10.75v-6.5z"/>
  <path d="M9.25 7A2.25 2.25 0 007 9.25v6.5A2.25 2.25 0 009.25 18h6.5A2.25 2.25 0 0018 15.75v-6.5A2.25 2.25 0 0015.75 7h-6.5z"/>
</svg>
  }
}
