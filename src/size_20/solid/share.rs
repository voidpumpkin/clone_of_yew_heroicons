use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M13%204%2E5a2%2E5%202%2E5%200%2011%2E702%201%2E737L6%2E97%209%2E604a2%2E518%202%2E518%200%20010%20%2E792l6%2E733%203%2E367a2%2E5%202%2E5%200%2011%2D%2E671%201%2E341l%2D6%2E733%2D3%2E367a2%2E5%202%2E5%200%20110%2D3%2E475l6%2E733%2D3%2E366A2%2E52%202%2E52%200%200113%204%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M13 4.5a2.5 2.5 0 11.702 1.737L6.97 9.604a2.518 2.518 0 010 .792l6.733 3.367a2.5 2.5 0 11-.671 1.341l-6.733-3.367a2.5 2.5 0 110-3.475l6.733-3.366A2.52 2.52 0 0113 4.5z"/>
</svg>
  }
}
