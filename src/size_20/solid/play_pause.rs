use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M12%2E75%204a%2E75%2E75%200%2000%2D%2E75%2E75v10%2E5c0%20%2E414%2E336%2E75%2E75%2E75h%2E5a%2E75%2E75%200%2000%2E75%2D%2E75V4%2E75a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D%2E5zM17%2E75%204a%2E75%2E75%200%2000%2D%2E75%2E75v10%2E5c0%20%2E414%2E336%2E75%2E75%2E75h%2E5a%2E75%2E75%200%2000%2E75%2D%2E75V4%2E75a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D%2E5zM3%2E288%204%2E819A1%2E5%201%2E5%200%20001%206%2E095v7%2E81a1%2E5%201%2E5%200%20002%2E288%201%2E277l6%2E323%2D3%2E906a1%2E5%201%2E5%200%20000%2D2%2E552L3%2E288%204%2E819z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PlayPauseIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M12.75 4a.75.75 0 00-.75.75v10.5c0 .414.336.75.75.75h.5a.75.75 0 00.75-.75V4.75a.75.75 0 00-.75-.75h-.5zM17.75 4a.75.75 0 00-.75.75v10.5c0 .414.336.75.75.75h.5a.75.75 0 00.75-.75V4.75a.75.75 0 00-.75-.75h-.5zM3.288 4.819A1.5 1.5 0 001 6.095v7.81a1.5 1.5 0 002.288 1.277l6.323-3.906a1.5 1.5 0 000-2.552L3.288 4.819z"/>
</svg>
  }
}