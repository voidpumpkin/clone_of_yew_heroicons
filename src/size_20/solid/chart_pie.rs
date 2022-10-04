use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M12%209a1%201%200%2001%2D1%2D1V3c0%2D%2E553%2E45%2D1%2E008%2E997%2D%2E93a7%2E004%207%2E004%200%20015%2E933%205%2E933c%2E078%2E547%2D%2E378%2E997%2D%2E93%2E997h%2D5z%22%2F%3E%20%3Cpath%20d%3D%22M8%2E003%204%2E07C8%2E55%203%2E992%209%204%2E447%209%205v5a1%201%200%20001%201h5c%2E552%200%201%2E008%2E45%2E93%2E997A7%2E001%207%2E001%200%20012%2011a7%2E002%207%2E002%200%20016%2E003%2D6%2E93z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChartPieIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M12 9a1 1 0 01-1-1V3c0-.553.45-1.008.997-.93a7.004 7.004 0 015.933 5.933c.078.547-.378.997-.93.997h-5z"/>
  <path d="M8.003 4.07C8.55 3.992 9 4.447 9 5v5a1 1 0 001 1h5c.552 0 1.008.45.93.997A7.001 7.001 0 012 11a7.002 7.002 0 016.003-6.93z"/>
</svg>
  }
}