use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M14%2E916%202%2E404a%2E75%2E75%200%2001%2D%2E32%201%2E012l%2D%2E596%2E31V17a1%201%200%2001%2D1%201h%2D2%2E26a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D3%2E5a%2E75%2E75%200%2000%2D%2E75%2D%2E75H6%2E75a%2E75%2E75%200%2000%2D%2E75%2E75v3%2E5a%2E75%2E75%200%2001%2D%2E75%2E75h%2D3%2E5a%2E75%2E75%200%20010%2D1%2E5H2V9%2E957a%2E75%2E75%200%2001%2D%2E596%2D1%2E372L2%208%2E275V5%2E75a%2E75%2E75%200%20011%2E5%200v1%2E745l10%2E404%2D5%2E41a%2E75%2E75%200%20011%2E012%2E32zM15%2E861%208%2E57a%2E75%2E75%200%2001%2E736%2D%2E025l1%2E999%201%2E04A%2E75%2E75%200%200118%2010%2E957V16%2E5h%2E25a%2E75%2E75%200%20110%201%2E5h%2D2a%2E75%2E75%200%2001%2D%2E75%2D%2E75V9%2E21a%2E75%2E75%200%2001%2E361%2D%2E64z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HomeModernIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M14.916 2.404a.75.75 0 01-.32 1.012l-.596.31V17a1 1 0 01-1 1h-2.26a.75.75 0 01-.75-.75v-3.5a.75.75 0 00-.75-.75H6.75a.75.75 0 00-.75.75v3.5a.75.75 0 01-.75.75h-3.5a.75.75 0 010-1.5H2V9.957a.75.75 0 01-.596-1.372L2 8.275V5.75a.75.75 0 011.5 0v1.745l10.404-5.41a.75.75 0 011.012.32zM15.861 8.57a.75.75 0 01.736-.025l1.999 1.04A.75.75 0 0118 10.957V16.5h.25a.75.75 0 110 1.5h-2a.75.75 0 01-.75-.75V9.21a.75.75 0 01.361-.64z"/>
</svg>
  }
}
