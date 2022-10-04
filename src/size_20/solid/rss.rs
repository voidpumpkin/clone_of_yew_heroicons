use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E75%203a%2E75%2E75%200%2000%2D%2E75%2E75v%2E5c0%20%2E414%2E336%2E75%2E75%2E75H4c6%2E075%200%2011%204%2E925%2011%2011v%2E25c0%20%2E414%2E336%2E75%2E75%2E75h%2E5a%2E75%2E75%200%2000%2E75%2D%2E75V16C17%208%2E82%2011%2E18%203%204%203h%2D%2E25z%22%2F%3E%20%3Cpath%20d%3D%22M3%208%2E75A%2E75%2E75%200%20013%2E75%208H4a8%208%200%20018%208v%2E25a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75V16a6%206%200%2000%2D6%2D6h%2D%2E25A%2E75%2E75%200%20013%209%2E25v%2D%2E5zM7%2015a2%202%200%2011%2D4%200%202%202%200%20014%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RssIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M3.75 3a.75.75 0 00-.75.75v.5c0 .414.336.75.75.75H4c6.075 0 11 4.925 11 11v.25c0 .414.336.75.75.75h.5a.75.75 0 00.75-.75V16C17 8.82 11.18 3 4 3h-.25z"/>
  <path d="M3 8.75A.75.75 0 013.75 8H4a8 8 0 018 8v.25a.75.75 0 01-.75.75h-.5a.75.75 0 01-.75-.75V16a6 6 0 00-6-6h-.25A.75.75 0 013 9.25v-.5zM7 15a2 2 0 11-4 0 2 2 0 014 0z"/>
</svg>
  }
}
