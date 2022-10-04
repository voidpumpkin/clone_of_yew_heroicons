use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%204%2E5A%2E75%2E75%200%20013%203%2E75h14%2E25a%2E75%2E75%200%20010%201%2E5H3a%2E75%2E75%200%2001%2D%2E75%2D%2E75zm14%2E47%203%2E97a%2E75%2E75%200%20011%2E06%200l3%2E75%203%2E75a%2E75%2E75%200%2011%2D1%2E06%201%2E06L18%2010%2E81V21a%2E75%2E75%200%2001%2D1%2E5%200V10%2E81l%2D2%2E47%202%2E47a%2E75%2E75%200%2011%2D1%2E06%2D1%2E06l3%2E75%2D3%2E75zM2%2E25%209A%2E75%2E75%200%20013%208%2E25h9%2E75a%2E75%2E75%200%20010%201%2E5H3A%2E75%2E75%200%20012%2E25%209zm0%204%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h5%2E25a%2E75%2E75%200%20010%201%2E5H3a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BarsArrowUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 4.5A.75.75 0 013 3.75h14.25a.75.75 0 010 1.5H3a.75.75 0 01-.75-.75zm14.47 3.97a.75.75 0 011.06 0l3.75 3.75a.75.75 0 11-1.06 1.06L18 10.81V21a.75.75 0 01-1.5 0V10.81l-2.47 2.47a.75.75 0 11-1.06-1.06l3.75-3.75zM2.25 9A.75.75 0 013 8.25h9.75a.75.75 0 010 1.5H3A.75.75 0 012.25 9zm0 4.5a.75.75 0 01.75-.75h5.25a.75.75 0 010 1.5H3a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}