use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E25%204A2%2E25%202%2E25%200%20001%206%2E25v7%2E5A2%2E25%202%2E25%200%20003%2E25%2016h7%2E5A2%2E25%202%2E25%200%200013%2013%2E75v%2D7%2E5A2%2E25%202%2E25%200%200010%2E75%204h%2D7%2E5zM19%204%2E75a%2E75%2E75%200%2000%2D1%2E28%2D%2E53l%2D3%203a%2E75%2E75%200%2000%2D%2E22%2E53v4%2E5c0%20%2E199%2E079%2E39%2E22%2E53l3%203a%2E75%2E75%200%20001%2E28%2D%2E53V4%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn VideoCameraIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M3.25 4A2.25 2.25 0 001 6.25v7.5A2.25 2.25 0 003.25 16h7.5A2.25 2.25 0 0013 13.75v-7.5A2.25 2.25 0 0010.75 4h-7.5zM19 4.75a.75.75 0 00-1.28-.53l-3 3a.75.75 0 00-.22.53v4.5c0 .199.079.39.22.53l3 3a.75.75 0 001.28-.53V4.75z"/>
</svg>
  }
}