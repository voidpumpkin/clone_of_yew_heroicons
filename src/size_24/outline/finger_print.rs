use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M7%2E864%204%2E243A7%2E5%207%2E5%200%200119%2E5%2010%2E5c0%202%2E92%2D%2E556%205%2E709%2D1%2E568%208%2E268M5%2E742%206%2E364A7%2E465%207%2E465%200%20004%2E5%2010%2E5a7%2E464%207%2E464%200%2001%2D1%2E15%203%2E993m1%2E989%203%2E559A11%2E209%2011%2E209%200%20008%2E25%2010%2E5a3%2E75%203%2E75%200%20117%2E5%200c0%20%2E527%2D%2E021%201%2E049%2D%2E064%201%2E565M12%2010%2E5a14%2E94%2014%2E94%200%2001%2D3%2E6%209%2E75m6%2E633%2D4%2E596a18%2E666%2018%2E666%200%2001%2D2%2E485%205%2E33%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FingerPrintIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M7.864 4.243A7.5 7.5 0 0119.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 004.5 10.5a7.464 7.464 0 01-1.15 3.993m1.989 3.559A11.209 11.209 0 008.25 10.5a3.75 3.75 0 117.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 01-3.6 9.75m6.633-4.596a18.666 18.666 0 01-2.485 5.33"/>
</svg>
  }
}