use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%2E5%204%2E5a3%203%200%20013%2D3h1%2E372c%2E86%200%201%2E61%2E586%201%2E819%201%2E42l1%2E105%204%2E423a1%2E875%201%2E875%200%2001%2D%2E694%201%2E955l%2D1%2E293%2E97c%2D%2E135%2E101%2D%2E164%2E249%2D%2E126%2E352a11%2E285%2011%2E285%200%20006%2E697%206%2E697c%2E103%2E038%2E25%2E009%2E352%2D%2E126l%2E97%2D1%2E293a1%2E875%201%2E875%200%20011%2E955%2D%2E694l4%2E423%201%2E105c%2E834%2E209%201%2E42%2E959%201%2E42%201%2E82V19%2E5a3%203%200%2001%2D3%203h%2D2%2E25C8%2E552%2022%2E5%201%2E5%2015%2E448%201%2E5%206%2E75V4%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PhoneIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1.5 4.5a3 3 0 013-3h1.372c.86 0 1.61.586 1.819 1.42l1.105 4.423a1.875 1.875 0 01-.694 1.955l-1.293.97c-.135.101-.164.249-.126.352a11.285 11.285 0 006.697 6.697c.103.038.25.009.352-.126l.97-1.293a1.875 1.875 0 011.955-.694l4.423 1.105c.834.209 1.42.959 1.42 1.82V19.5a3 3 0 01-3 3h-2.25C8.552 22.5 1.5 15.448 1.5 6.75V4.5z" clip-rule="evenodd"/>
</svg>
  }
}