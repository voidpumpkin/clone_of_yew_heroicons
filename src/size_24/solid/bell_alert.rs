use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M5%2E85%203%2E5a%2E75%2E75%200%2000%2D1%2E117%2D1%209%2E719%209%2E719%200%2000%2D2%2E348%204%2E876%2E75%2E75%200%20001%2E479%2E248A8%2E219%208%2E219%200%20015%2E85%203%2E5zM19%2E267%202%2E5a%2E75%2E75%200%2010%2D1%2E118%201%208%2E22%208%2E22%200%20011%2E987%204%2E124%2E75%2E75%200%20001%2E48%2D%2E248A9%2E72%209%2E72%200%200019%2E266%202%2E5z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%202%2E25A6%2E75%206%2E75%200%20005%2E25%209v%2E75a8%2E217%208%2E217%200%2001%2D2%2E119%205%2E52%2E75%2E75%200%2000%2E298%201%2E206c1%2E544%2E57%203%2E16%2E99%204%2E831%201%2E243a3%2E75%203%2E75%200%20107%2E48%200%2024%2E583%2024%2E583%200%20004%2E83%2D1%2E244%2E75%2E75%200%2000%2E298%2D1%2E205%208%2E217%208%2E217%200%2001%2D2%2E118%2D5%2E52V9A6%2E75%206%2E75%200%200012%202%2E25zM9%2E75%2018c0%2D%2E034%200%2D%2E067%2E002%2D%2E1a25%2E05%2025%2E05%200%20004%2E496%200l%2E002%2E1a2%2E25%202%2E25%200%2011%2D4%2E5%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BellAlertIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M5.85 3.5a.75.75 0 00-1.117-1 9.719 9.719 0 00-2.348 4.876.75.75 0 001.479.248A8.219 8.219 0 015.85 3.5zM19.267 2.5a.75.75 0 10-1.118 1 8.22 8.22 0 011.987 4.124.75.75 0 001.48-.248A9.72 9.72 0 0019.266 2.5z"/>
  <path fill-rule="evenodd" d="M12 2.25A6.75 6.75 0 005.25 9v.75a8.217 8.217 0 01-2.119 5.52.75.75 0 00.298 1.206c1.544.57 3.16.99 4.831 1.243a3.75 3.75 0 107.48 0 24.583 24.583 0 004.83-1.244.75.75 0 00.298-1.205 8.217 8.217 0 01-2.118-5.52V9A6.75 6.75 0 0012 2.25zM9.75 18c0-.034 0-.067.002-.1a25.05 25.05 0 004.496 0l.002.1a2.25 2.25 0 11-4.5 0z" clip-rule="evenodd"/>
</svg>
  }
}