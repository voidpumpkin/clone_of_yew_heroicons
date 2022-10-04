use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M9%2E195%2018%2E44c1%2E25%2E713%202%2E805%2D%2E19%202%2E805%2D1%2E629v%2D2%2E34l6%2E945%203%2E968c1%2E25%2E714%202%2E805%2D%2E188%202%2E805%2D1%2E628V8%2E688c0%2D1%2E44%2D1%2E555%2D2%2E342%2D2%2E805%2D1%2E628L12%2011%2E03v%2D2%2E34c0%2D1%2E44%2D1%2E555%2D2%2E343%2D2%2E805%2D1%2E629l%2D7%2E108%204%2E062c%2D1%2E26%2E72%2D1%2E26%202%2E536%200%203%2E256l7%2E108%204%2E061z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BackwardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M9.195 18.44c1.25.713 2.805-.19 2.805-1.629v-2.34l6.945 3.968c1.25.714 2.805-.188 2.805-1.628V8.688c0-1.44-1.555-2.342-2.805-1.628L12 11.03v-2.34c0-1.44-1.555-2.343-2.805-1.629l-7.108 4.062c-1.26.72-1.26 2.536 0 3.256l7.108 4.061z"/>
</svg>
  }
}