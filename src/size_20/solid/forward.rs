use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E288%204%2E819A1%2E5%201%2E5%200%20001%206%2E095v7%2E81a1%2E5%201%2E5%200%20002%2E288%201%2E277l6%2E323%2D3%2E905c%2E155%2D%2E096%2E285%2D%2E213%2E389%2D%2E344v2%2E973a1%2E5%201%2E5%200%20002%2E288%201%2E276l6%2E323%2D3%2E905a1%2E5%201%2E5%200%20000%2D2%2E553L12%2E288%204%2E82A1%2E5%201%2E5%200%200010%206%2E095v2%2E973a1%2E506%201%2E506%200%2000%2D%2E389%2D%2E344L3%2E288%204%2E82z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ForwardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M3.288 4.819A1.5 1.5 0 001 6.095v7.81a1.5 1.5 0 002.288 1.277l6.323-3.905c.155-.096.285-.213.389-.344v2.973a1.5 1.5 0 002.288 1.276l6.323-3.905a1.5 1.5 0 000-2.553L12.288 4.82A1.5 1.5 0 0010 6.095v2.973a1.506 1.506 0 00-.389-.344L3.288 4.82z"/>
</svg>
  }
}
