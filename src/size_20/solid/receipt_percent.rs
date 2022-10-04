use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%2E93%201%2E31a41%2E401%2041%2E401%200%200110%2E14%200C16%2E194%201%2E45%2017%202%2E414%2017%203%2E517V18%2E25a%2E75%2E75%200%2001%2D1%2E075%2E676l%2D2%2E8%2D1%2E344%2D2%2E8%201%2E344a%2E75%2E75%200%2001%2D%2E65%200l%2D2%2E8%2D1%2E344%2D2%2E8%201%2E344A%2E75%2E75%200%20013%2018%2E25V3%2E517c0%2D1%2E103%2E806%2D2%2E068%201%2E93%2D2%2E207zm8%2E85%205%2E97a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06l%2D6%2E5%206%2E5a%2E75%2E75%200%20101%2E06%201%2E06l6%2E5%2D6%2E5zM9%208a1%201%200%2011%2D2%200%201%201%200%20012%200zm3%205a1%201%200%20100%2D2%201%201%200%20000%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ReceiptPercentIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M4.93 1.31a41.401 41.401 0 0110.14 0C16.194 1.45 17 2.414 17 3.517V18.25a.75.75 0 01-1.075.676l-2.8-1.344-2.8 1.344a.75.75 0 01-.65 0l-2.8-1.344-2.8 1.344A.75.75 0 013 18.25V3.517c0-1.103.806-2.068 1.93-2.207zm8.85 5.97a.75.75 0 00-1.06-1.06l-6.5 6.5a.75.75 0 101.06 1.06l6.5-6.5zM9 8a1 1 0 11-2 0 1 1 0 012 0zm3 5a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
</svg>
  }
}