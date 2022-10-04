use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2E752%2015%2E002A9%2E718%209%2E718%200%200118%2015%2E75c%2D5%2E385%200%2D9%2E75%2D4%2E365%2D9%2E75%2D9%2E75%200%2D1%2E33%2E266%2D2%2E597%2E748%2D3%2E752A9%2E753%209%2E753%200%20003%2011%2E25C3%2016%2E635%207%2E365%2021%2012%2E75%2021a9%2E753%209%2E753%200%20009%2E002%2D5%2E998z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MoonIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z"/>
</svg>
  }
}
