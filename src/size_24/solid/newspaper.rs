use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%2E125%203C3%2E089%203%202%2E25%203%2E84%202%2E25%204%2E875V18a3%203%200%20003%203h15a3%203%200%2001%2D3%2D3V4%2E875C17%2E25%203%2E839%2016%2E41%203%2015%2E375%203H4%2E125zM12%209%2E75a%2E75%2E75%200%20000%201%2E5h1%2E5a%2E75%2E75%200%20000%2D1%2E5H12zm%2D%2E75%2D2%2E25a%2E75%2E75%200%2001%2E75%2D%2E75h1%2E5a%2E75%2E75%200%20010%201%2E5H12a%2E75%2E75%200%2001%2D%2E75%2D%2E75zM6%2012%2E75a%2E75%2E75%200%20000%201%2E5h7%2E5a%2E75%2E75%200%20000%2D1%2E5H6zm%2D%2E75%203%2E75a%2E75%2E75%200%2001%2E75%2D%2E75h7%2E5a%2E75%2E75%200%20010%201%2E5H6a%2E75%2E75%200%2001%2D%2E75%2D%2E75zM6%206%2E75a%2E75%2E75%200%2000%2D%2E75%2E75v3c0%20%2E414%2E336%2E75%2E75%2E75h3a%2E75%2E75%200%2000%2E75%2D%2E75v%2D3A%2E75%2E75%200%20009%206%2E75H6z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M18%2E75%206%2E75h1%2E875c%2E621%200%201%2E125%2E504%201%2E125%201%2E125V18a1%2E5%201%2E5%200%2001%2D3%200V6%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn NewspaperIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M4.125 3C3.089 3 2.25 3.84 2.25 4.875V18a3 3 0 003 3h15a3 3 0 01-3-3V4.875C17.25 3.839 16.41 3 15.375 3H4.125zM12 9.75a.75.75 0 000 1.5h1.5a.75.75 0 000-1.5H12zm-.75-2.25a.75.75 0 01.75-.75h1.5a.75.75 0 010 1.5H12a.75.75 0 01-.75-.75zM6 12.75a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5H6zm-.75 3.75a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5H6a.75.75 0 01-.75-.75zM6 6.75a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h3a.75.75 0 00.75-.75v-3A.75.75 0 009 6.75H6z" clip-rule="evenodd"/>
  <path d="M18.75 6.75h1.875c.621 0 1.125.504 1.125 1.125V18a1.5 1.5 0 01-3 0V6.75z"/>
</svg>
  }
}