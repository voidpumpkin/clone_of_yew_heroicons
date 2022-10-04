use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E5%204%2E5a3%203%200%2000%2D3%203v9a3%203%200%20003%203h8%2E25a3%203%200%20003%2D3v%2D9a3%203%200%2000%2D3%2D3H4%2E5zM19%2E94%2018%2E75l%2D2%2E69%2D2%2E69V7%2E94l2%2E69%2D2%2E69c%2E944%2D%2E945%202%2E56%2D%2E276%202%2E56%201%2E06v11%2E38c0%201%2E336%2D1%2E616%202%2E005%2D2%2E56%201%2E06z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn VideoCameraIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M4.5 4.5a3 3 0 00-3 3v9a3 3 0 003 3h8.25a3 3 0 003-3v-9a3 3 0 00-3-3H4.5zM19.94 18.75l-2.69-2.69V7.94l2.69-2.69c.944-.945 2.56-.276 2.56 1.06v11.38c0 1.336-1.616 2.005-2.56 1.06z"/>
</svg>
  }
}
