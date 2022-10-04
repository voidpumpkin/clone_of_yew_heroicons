use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M16%2E098%202%2E598a3%2E75%203%2E75%200%20113%2E622%206%2E275l%2D1%2E72%2E46V12a%2E75%2E75%200%2001%2D%2E22%2E53l%2D%2E75%2E75a%2E75%2E75%200%2001%2D1%2E06%200l%2D%2E97%2D%2E97%2D7%2E94%207%2E94a2%2E56%202%2E56%200%2001%2D1%2E81%2E75%201%2E06%201%2E06%200%2000%2D%2E75%2E31l%2D%2E97%2E97a%2E75%2E75%200%2001%2D1%2E06%200l%2D%2E75%2D%2E75a%2E75%2E75%200%20010%2D1%2E06l%2E97%2D%2E97a1%2E06%201%2E06%200%2000%2E31%2D%2E75c0%2D%2E68%2E27%2D1%2E33%2E75%2D1%2E81L11%2E69%209l%2D%2E97%2D%2E97a%2E75%2E75%200%20010%2D1%2E06l%2E75%2D%2E75A%2E75%2E75%200%200112%206h2%2E666l%2E461%2D1%2E72c%2E165%2D%2E617%2E49%2D1%2E2%2E971%2D1%2E682zm%2D3%2E348%207%2E463L4%2E81%2018a1%2E06%201%2E06%200%2000%2D%2E31%2E75c0%20%2E318%2D%2E06%2E63%2D%2E172%2E922a2%2E56%202%2E56%200%2001%2E922%2D%2E172c%2E281%200%20%2E551%2D%2E112%2E75%2D%2E31l7%2E94%2D7%2E94%2D1%2E19%2D1%2E19z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EyeDropperIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M16.098 2.598a3.75 3.75 0 113.622 6.275l-1.72.46V12a.75.75 0 01-.22.53l-.75.75a.75.75 0 01-1.06 0l-.97-.97-7.94 7.94a2.56 2.56 0 01-1.81.75 1.06 1.06 0 00-.75.31l-.97.97a.75.75 0 01-1.06 0l-.75-.75a.75.75 0 010-1.06l.97-.97a1.06 1.06 0 00.31-.75c0-.68.27-1.33.75-1.81L11.69 9l-.97-.97a.75.75 0 010-1.06l.75-.75A.75.75 0 0112 6h2.666l.461-1.72c.165-.617.49-1.2.971-1.682zm-3.348 7.463L4.81 18a1.06 1.06 0 00-.31.75c0 .318-.06.63-.172.922a2.56 2.56 0 01.922-.172c.281 0 .551-.112.75-.31l7.94-7.94-1.19-1.19z" clip-rule="evenodd"/>
</svg>
  }
}