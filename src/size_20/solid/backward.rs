use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M7%2E712%204%2E819A1%2E5%201%2E5%200%200110%206%2E095v2%2E973c%2E104%2D%2E131%2E234%2D%2E248%2E389%2D%2E344l6%2E323%2D3%2E905A1%2E5%201%2E5%200%200119%206%2E095v7%2E81a1%2E5%201%2E5%200%2001%2D2%2E288%201%2E277l%2D6%2E323%2D3%2E905a1%2E505%201%2E505%200%2001%2D%2E389%2D%2E344v2%2E973a1%2E5%201%2E5%200%2001%2D2%2E288%201%2E276l%2D6%2E323%2D3%2E905a1%2E5%201%2E5%200%20010%2D2%2E553L7%2E712%204%2E82z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BackwardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M7.712 4.819A1.5 1.5 0 0110 6.095v2.973c.104-.131.234-.248.389-.344l6.323-3.905A1.5 1.5 0 0119 6.095v7.81a1.5 1.5 0 01-2.288 1.277l-6.323-3.905a1.505 1.505 0 01-.389-.344v2.973a1.5 1.5 0 01-2.288 1.276l-6.323-3.905a1.5 1.5 0 010-2.553L7.712 4.82z"/>
</svg>
  }
}