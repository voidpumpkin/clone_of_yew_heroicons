use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M11%202a1%201%200%2010%2D2%200v6%2E5a%2E5%2E5%200%2001%2D1%200V3a1%201%200%2010%2D2%200v5%2E5a%2E5%2E5%200%2001%2D1%200V5a1%201%200%2010%2D2%200v7a7%207%200%201014%200V8a1%201%200%2010%2D2%200v3%2E5a%2E5%2E5%200%2001%2D1%200V3a1%201%200%2010%2D2%200v5%2E5a%2E5%2E5%200%2001%2D1%200V2z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HandRaisedIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M11 2a1 1 0 10-2 0v6.5a.5.5 0 01-1 0V3a1 1 0 10-2 0v5.5a.5.5 0 01-1 0V5a1 1 0 10-2 0v7a7 7 0 1014 0V8a1 1 0 10-2 0v3.5a.5.5 0 01-1 0V3a1 1 0 10-2 0v5.5a.5.5 0 01-1 0V2z" clip-rule="evenodd"/>
</svg>
  }
}
