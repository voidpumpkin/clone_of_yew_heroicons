use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M11%205a3%203%200%2011%2D6%200%203%203%200%20016%200zM2%2E615%2016%2E428a1%2E224%201%2E224%200%2001%2D%2E569%2D1%2E175%206%2E002%206%2E002%200%200111%2E908%200c%2E058%2E467%2D%2E172%2E92%2D%2E57%201%2E174A9%2E953%209%2E953%200%20018%2018a9%2E953%209%2E953%200%2001%2D5%2E385%2D1%2E572zM16%2E25%205%2E75a%2E75%2E75%200%2000%2D1%2E5%200v2h%2D2a%2E75%2E75%200%20000%201%2E5h2v2a%2E75%2E75%200%20001%2E5%200v%2D2h2a%2E75%2E75%200%20000%2D1%2E5h%2D2v%2D2z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserPlusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M11 5a3 3 0 11-6 0 3 3 0 016 0zM2.615 16.428a1.224 1.224 0 01-.569-1.175 6.002 6.002 0 0111.908 0c.058.467-.172.92-.57 1.174A9.953 9.953 0 018 18a9.953 9.953 0 01-5.385-1.572zM16.25 5.75a.75.75 0 00-1.5 0v2h-2a.75.75 0 000 1.5h2v2a.75.75 0 001.5 0v-2h2a.75.75 0 000-1.5h-2v-2z"/>
</svg>
  }
}