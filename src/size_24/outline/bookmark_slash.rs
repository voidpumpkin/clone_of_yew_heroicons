use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%203l1%2E664%201%2E664M21%2021l%2D1%2E5%2D1%2E5m%2D5%2E485%2D1%2E242L12%2017%2E25%204%2E5%2021V8%2E742m%2E164%2D4%2E078a2%2E15%202%2E15%200%20011%2E743%2D1%2E342%2048%2E507%2048%2E507%200%200111%2E186%200c1%2E1%2E128%201%2E907%201%2E077%201%2E907%202%2E185V19%2E5M4%2E664%204%2E664L19%2E5%2019%2E5%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BookmarkSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3 3l1.664 1.664M21 21l-1.5-1.5m-5.485-1.242L12 17.25 4.5 21V8.742m.164-4.078a2.15 2.15 0 011.743-1.342 48.507 48.507 0 0111.186 0c1.1.128 1.907 1.077 1.907 2.185V19.5M4.664 4.664L19.5 19.5"/>
</svg>
  }
}