use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M21%2E731%202%2E269a2%2E625%202%2E625%200%2000%2D3%2E712%200l%2D1%2E157%201%2E157%203%2E712%203%2E712%201%2E157%2D1%2E157a2%2E625%202%2E625%200%20000%2D3%2E712zM19%2E513%208%2E199l%2D3%2E712%2D3%2E712%2D12%2E15%2012%2E15a5%2E25%205%2E25%200%2000%2D1%2E32%202%2E214l%2D%2E8%202%2E685a%2E75%2E75%200%2000%2E933%2E933l2%2E685%2D%2E8a5%2E25%205%2E25%200%20002%2E214%2D1%2E32L19%2E513%208%2E2z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PencilIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M21.731 2.269a2.625 2.625 0 00-3.712 0l-1.157 1.157 3.712 3.712 1.157-1.157a2.625 2.625 0 000-3.712zM19.513 8.199l-3.712-3.712-12.15 12.15a5.25 5.25 0 00-1.32 2.214l-.8 2.685a.75.75 0 00.933.933l2.685-.8a5.25 5.25 0 002.214-1.32L19.513 8.2z"/>
</svg>
  }
}