use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E478%202%2E405a%2E75%2E75%200%2000%2D%2E926%2E94l2%2E432%207%2E905H13%2E5a%2E75%2E75%200%20010%201%2E5H4%2E984l%2D2%2E432%207%2E905a%2E75%2E75%200%2000%2E926%2E94%2060%2E519%2060%2E519%200%200018%2E445%2D8%2E986%2E75%2E75%200%20000%2D1%2E218A60%2E517%2060%2E517%200%20003%2E478%202%2E405z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PaperAirplaneIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M3.478 2.405a.75.75 0 00-.926.94l2.432 7.905H13.5a.75.75 0 010 1.5H4.984l-2.432 7.905a.75.75 0 00.926.94 60.519 60.519 0 0018.445-8.986.75.75 0 000-1.218A60.517 60.517 0 003.478 2.405z"/>
</svg>
  }
}