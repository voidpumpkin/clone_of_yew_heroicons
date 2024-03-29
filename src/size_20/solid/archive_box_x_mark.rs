use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M2%203a1%201%200%2000%2D1%201v1a1%201%200%20001%201h16a1%201%200%20001%2D1V4a1%201%200%2000%2D1%2D1H2z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%207%2E5h16l%2D%2E811%207%2E71a2%202%200%2001%2D1%2E99%201%2E79H4%2E802a2%202%200%2001%2D1%2E99%2D1%2E79L2%207%2E5zm5%2E22%201%2E72a%2E75%2E75%200%20011%2E06%200L10%2010%2E94l1%2E72%2D1%2E72a%2E75%2E75%200%20111%2E06%201%2E06L11%2E06%2012l1%2E72%201%2E72a%2E75%2E75%200%2011%2D1%2E06%201%2E06L10%2013%2E06l%2D1%2E72%201%2E72a%2E75%2E75%200%2001%2D1%2E06%2D1%2E06L8%2E94%2012l%2D1%2E72%2D1%2E72a%2E75%2E75%200%20010%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArchiveBoxXMarkIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M2 3a1 1 0 00-1 1v1a1 1 0 001 1h16a1 1 0 001-1V4a1 1 0 00-1-1H2z"/>
  <path fill-rule="evenodd" d="M2 7.5h16l-.811 7.71a2 2 0 01-1.99 1.79H4.802a2 2 0 01-1.99-1.79L2 7.5zm5.22 1.72a.75.75 0 011.06 0L10 10.94l1.72-1.72a.75.75 0 111.06 1.06L11.06 12l1.72 1.72a.75.75 0 11-1.06 1.06L10 13.06l-1.72 1.72a.75.75 0 01-1.06-1.06L8.94 12l-1.72-1.72a.75.75 0 010-1.06z" clip-rule="evenodd"/>
</svg>
  }
}
