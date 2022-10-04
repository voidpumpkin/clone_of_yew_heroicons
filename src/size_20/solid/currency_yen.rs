use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2018a8%208%200%20100%2D16%208%208%200%20000%2016zM7%2E346%205%2E294a%2E75%2E75%200%2000%2D1%2E192%2E912L9%2E056%2010H6%2E75a%2E75%2E75%200%20000%201%2E5h2%2E5v1h%2D2%2E5a%2E75%2E75%200%20000%201%2E5h2%2E5v1%2E25a%2E75%2E75%200%20001%2E5%200V14h2%2E5a%2E75%2E75%200%20100%2D1%2E5h%2D2%2E5v%2D1h2%2E5a%2E75%2E75%200%20100%2D1%2E5h%2D2%2E306l2%2E902%2D3%2E794a%2E75%2E75%200%2010%2D1%2E192%2D%2E912L10%208%2E765l%2D2%2E654%2D3%2E47z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CurrencyYenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM7.346 5.294a.75.75 0 00-1.192.912L9.056 10H6.75a.75.75 0 000 1.5h2.5v1h-2.5a.75.75 0 000 1.5h2.5v1.25a.75.75 0 001.5 0V14h2.5a.75.75 0 100-1.5h-2.5v-1h2.5a.75.75 0 100-1.5h-2.306l2.902-3.794a.75.75 0 10-1.192-.912L10 8.765l-2.654-3.47z" clip-rule="evenodd"/>
</svg>
  }
}