use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M21%2E731%202%2E269a2%2E625%202%2E625%200%2000%2D3%2E712%200l%2D1%2E157%201%2E157%203%2E712%203%2E712%201%2E157%2D1%2E157a2%2E625%202%2E625%200%20000%2D3%2E712zM19%2E513%208%2E199l%2D3%2E712%2D3%2E712%2D8%2E4%208%2E4a5%2E25%205%2E25%200%2000%2D1%2E32%202%2E214l%2D%2E8%202%2E685a%2E75%2E75%200%2000%2E933%2E933l2%2E685%2D%2E8a5%2E25%205%2E25%200%20002%2E214%2D1%2E32l8%2E4%2D8%2E4z%22%2F%3E%20%3Cpath%20d%3D%22M5%2E25%205%2E25a3%203%200%2000%2D3%203v10%2E5a3%203%200%20003%203h10%2E5a3%203%200%20003%2D3V13%2E5a%2E75%2E75%200%2000%2D1%2E5%200v5%2E25a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5H5%2E25a1%2E5%201%2E5%200%2001%2D1%2E5%2D1%2E5V8%2E25a1%2E5%201%2E5%200%20011%2E5%2D1%2E5h5%2E25a%2E75%2E75%200%20000%2D1%2E5H5%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PencilSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M21.731 2.269a2.625 2.625 0 00-3.712 0l-1.157 1.157 3.712 3.712 1.157-1.157a2.625 2.625 0 000-3.712zM19.513 8.199l-3.712-3.712-8.4 8.4a5.25 5.25 0 00-1.32 2.214l-.8 2.685a.75.75 0 00.933.933l2.685-.8a5.25 5.25 0 002.214-1.32l8.4-8.4z"/>
  <path d="M5.25 5.25a3 3 0 00-3 3v10.5a3 3 0 003 3h10.5a3 3 0 003-3V13.5a.75.75 0 00-1.5 0v5.25a1.5 1.5 0 01-1.5 1.5H5.25a1.5 1.5 0 01-1.5-1.5V8.25a1.5 1.5 0 011.5-1.5h5.25a.75.75 0 000-1.5H5.25z"/>
</svg>
  }
}