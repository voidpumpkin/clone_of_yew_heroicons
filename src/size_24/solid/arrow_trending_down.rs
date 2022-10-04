use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%2E72%205%2E47a%2E75%2E75%200%20011%2E06%200L9%2011%2E69l3%2E756%2D3%2E756a%2E75%2E75%200%2001%2E985%2D%2E066%2012%2E698%2012%2E698%200%20014%2E575%206%2E832l%2E308%201%2E149%202%2E277%2D3%2E943a%2E75%2E75%200%20111%2E299%2E75l%2D3%2E182%205%2E51a%2E75%2E75%200%2001%2D1%2E025%2E275l%2D5%2E511%2D3%2E181a%2E75%2E75%200%2001%2E75%2D1%2E3l3%2E943%202%2E277%2D%2E308%2D1%2E149a11%2E194%2011%2E194%200%2000%2D3%2E528%2D5%2E617l%2D3%2E809%203%2E81a%2E75%2E75%200%2001%2D1%2E06%200L1%2E72%206%2E53a%2E75%2E75%200%20010%2D1%2E061z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowTrendingDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1.72 5.47a.75.75 0 011.06 0L9 11.69l3.756-3.756a.75.75 0 01.985-.066 12.698 12.698 0 014.575 6.832l.308 1.149 2.277-3.943a.75.75 0 111.299.75l-3.182 5.51a.75.75 0 01-1.025.275l-5.511-3.181a.75.75 0 01.75-1.3l3.943 2.277-.308-1.149a11.194 11.194 0 00-3.528-5.617l-3.809 3.81a.75.75 0 01-1.06 0L1.72 6.53a.75.75 0 010-1.061z" clip-rule="evenodd"/>
</svg>
  }
}