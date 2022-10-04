use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M16%2E364%203%2E636a%2E75%2E75%200%2000%2D1%2E06%201%2E06%207%2E5%207%2E5%200%20010%2010%2E607%2E75%2E75%200%20001%2E06%201%2E061%209%209%200%20000%2D12%2E728zM4%2E697%204%2E697a%2E75%2E75%200%2000%2D1%2E061%2D1%2E06%209%209%200%20000%2012%2E727%2E75%2E75%200%20101%2E06%2D1%2E06%207%2E5%207%2E5%200%20010%2D10%2E607z%22%2F%3E%20%3Cpath%20d%3D%22M12%2E475%206%2E465a%2E75%2E75%200%20011%2E06%200%205%205%200%20010%207%2E07%2E75%2E75%200%2011%2D1%2E06%2D1%2E06%203%2E5%203%2E5%200%20000%2D4%2E95%2E75%2E75%200%20010%2D1%2E06zM7%2E525%206%2E465a%2E75%2E75%200%20010%201%2E06%203%2E5%203%2E5%200%20000%204%2E95%2E75%2E75%200%2001%2D1%2E06%201%2E06%205%205%200%20010%2D7%2E07%2E75%2E75%200%20011%2E06%200zM11%2010a1%201%200%2011%2D2%200%201%201%200%20012%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SignalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M16.364 3.636a.75.75 0 00-1.06 1.06 7.5 7.5 0 010 10.607.75.75 0 001.06 1.061 9 9 0 000-12.728zM4.697 4.697a.75.75 0 00-1.061-1.06 9 9 0 000 12.727.75.75 0 101.06-1.06 7.5 7.5 0 010-10.607z"/>
  <path d="M12.475 6.465a.75.75 0 011.06 0 5 5 0 010 7.07.75.75 0 11-1.06-1.06 3.5 3.5 0 000-4.95.75.75 0 010-1.06zM7.525 6.465a.75.75 0 010 1.06 3.5 3.5 0 000 4.95.75.75 0 01-1.06 1.06 5 5 0 010-7.07.75.75 0 011.06 0zM11 10a1 1 0 11-2 0 1 1 0 012 0z"/>
</svg>
  }
}