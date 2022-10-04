use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E792%202%2E938A49%2E069%2049%2E069%200%200112%202%2E25c2%2E797%200%205%2E54%2E236%208%2E209%2E688a1%2E857%201%2E857%200%20011%2E541%201%2E836v1%2E044a3%203%200%2001%2D%2E879%202%2E121l%2D6%2E182%206%2E182a1%2E5%201%2E5%200%2000%2D%2E439%201%2E061v2%2E927a3%203%200%2001%2D1%2E658%202%2E684l%2D1%2E757%2E878A%2E75%2E75%200%20019%2E75%2021v%2D5%2E818a1%2E5%201%2E5%200%2000%2D%2E44%2D1%2E06L3%2E13%207%2E938a3%203%200%2001%2D%2E879%2D2%2E121V4%2E774c0%2D%2E897%2E64%2D1%2E683%201%2E542%2D1%2E836z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FunnelIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.792 2.938A49.069 49.069 0 0112 2.25c2.797 0 5.54.236 8.209.688a1.857 1.857 0 011.541 1.836v1.044a3 3 0 01-.879 2.121l-6.182 6.182a1.5 1.5 0 00-.439 1.061v2.927a3 3 0 01-1.658 2.684l-1.757.878A.75.75 0 019.75 21v-5.818a1.5 1.5 0 00-.44-1.06L3.13 7.938a3 3 0 01-.879-2.121V4.774c0-.897.64-1.683 1.542-1.836z" clip-rule="evenodd"/>
</svg>
  }
}
