use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E53%202%2E47a%2E75%2E75%200%2000%2D1%2E06%201%2E06l18%2018a%2E75%2E75%200%20101%2E06%2D1%2E06l%2D18%2D18zM20%2E25%205%2E507v11%2E561L5%2E853%202%2E671c%2E15%2D%2E043%2E306%2D%2E075%2E467%2D%2E094a49%2E255%2049%2E255%200%200111%2E36%200c1%2E497%2E174%202%2E57%201%2E46%202%2E57%202%2E93zM3%2E75%2021V6%2E932l14%2E063%2014%2E063L12%2018%2E088l%2D7%2E165%203%2E583A%2E75%2E75%200%20013%2E75%2021z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BookmarkSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M3.53 2.47a.75.75 0 00-1.06 1.06l18 18a.75.75 0 101.06-1.06l-18-18zM20.25 5.507v11.561L5.853 2.671c.15-.043.306-.075.467-.094a49.255 49.255 0 0111.36 0c1.497.174 2.57 1.46 2.57 2.93zM3.75 21V6.932l14.063 14.063L12 18.088l-7.165 3.583A.75.75 0 013.75 21z"/>
</svg>
  }
}