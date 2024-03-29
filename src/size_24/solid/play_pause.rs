use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M15%206%2E75a%2E75%2E75%200%2000%2D%2E75%2E75V18a%2E75%2E75%200%2000%2E75%2E75h%2E75a%2E75%2E75%200%2000%2E75%2D%2E75V7%2E5a%2E75%2E75%200%2000%2D%2E75%2D%2E75H15zM20%2E25%206%2E75a%2E75%2E75%200%2000%2D%2E75%2E75V18c0%20%2E414%2E336%2E75%2E75%2E75H21a%2E75%2E75%200%2000%2E75%2D%2E75V7%2E5a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D%2E75zM5%2E055%207%2E06C3%2E805%206%2E347%202%2E25%207%2E25%202%2E25%208%2E69v8%2E122c0%201%2E44%201%2E555%202%2E343%202%2E805%201%2E628l7%2E108%2D4%2E061c1%2E26%2D%2E72%201%2E26%2D2%2E536%200%2D3%2E256L5%2E055%207%2E061z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PlayPauseIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M15 6.75a.75.75 0 00-.75.75V18a.75.75 0 00.75.75h.75a.75.75 0 00.75-.75V7.5a.75.75 0 00-.75-.75H15zM20.25 6.75a.75.75 0 00-.75.75V18c0 .414.336.75.75.75H21a.75.75 0 00.75-.75V7.5a.75.75 0 00-.75-.75h-.75zM5.055 7.06C3.805 6.347 2.25 7.25 2.25 8.69v8.122c0 1.44 1.555 2.343 2.805 1.628l7.108-4.061c1.26-.72 1.26-2.536 0-3.256L5.055 7.061z"/>
</svg>
  }
}
