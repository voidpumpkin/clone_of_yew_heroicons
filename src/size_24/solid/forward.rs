use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M5%2E055%207%2E06c%2D1%2E25%2D%2E714%2D2%2E805%2E189%2D2%2E805%201%2E628v8%2E123c0%201%2E44%201%2E555%202%2E342%202%2E805%201%2E628L12%2014%2E471v2%2E34c0%201%2E44%201%2E555%202%2E342%202%2E805%201%2E628l7%2E108%2D4%2E061c1%2E26%2D%2E72%201%2E26%2D2%2E536%200%2D3%2E256L14%2E805%207%2E06C13%2E555%206%2E346%2012%207%2E25%2012%208%2E688v2%2E34L5%2E055%207%2E06z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ForwardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M5.055 7.06c-1.25-.714-2.805.189-2.805 1.628v8.123c0 1.44 1.555 2.342 2.805 1.628L12 14.471v2.34c0 1.44 1.555 2.342 2.805 1.628l7.108-4.061c1.26-.72 1.26-2.536 0-3.256L14.805 7.06C13.555 6.346 12 7.25 12 8.688v2.34L5.055 7.06z"/>
</svg>
  }
}