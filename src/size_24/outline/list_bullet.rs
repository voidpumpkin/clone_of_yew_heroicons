use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E25%206%2E75h12M8%2E25%2012h12m%2D12%205%2E25h12M3%2E75%206%2E75h%2E007v%2E008H3%2E75V6%2E75zm%2E375%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200zM3%2E75%2012h%2E007v%2E008H3%2E75V12zm%2E375%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200zm%2D%2E375%205%2E25h%2E007v%2E008H3%2E75v%2D%2E008zm%2E375%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ListBulletIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"/>
</svg>
  }
}
