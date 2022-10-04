use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M6%2E827%206%2E175A2%2E31%202%2E31%200%20015%2E186%207%2E23c%2D%2E38%2E054%2D%2E757%2E112%2D1%2E134%2E175C2%2E999%207%2E58%202%2E25%208%2E507%202%2E25%209%2E574V18a2%2E25%202%2E25%200%20002%2E25%202%2E25h15A2%2E25%202%2E25%200%200021%2E75%2018V9%2E574c0%2D1%2E067%2D%2E75%2D1%2E994%2D1%2E802%2D2%2E169a47%2E865%2047%2E865%200%2000%2D1%2E134%2D%2E175%202%2E31%202%2E31%200%2001%2D1%2E64%2D1%2E055l%2D%2E822%2D1%2E316a2%2E192%202%2E192%200%2000%2D1%2E736%2D1%2E039%2048%2E774%2048%2E774%200%2000%2D5%2E232%200%202%2E192%202%2E192%200%2000%2D1%2E736%201%2E039l%2D%2E821%201%2E316z%22%2F%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M16%2E5%2012%2E75a4%2E5%204%2E5%200%2011%2D9%200%204%2E5%204%2E5%200%20019%200zM18%2E75%2010%2E5h%2E008v%2E008h%2D%2E008V10%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CameraIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M6.827 6.175A2.31 2.31 0 015.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 00-1.134-.175 2.31 2.31 0 01-1.64-1.055l-.822-1.316a2.192 2.192 0 00-1.736-1.039 48.774 48.774 0 00-5.232 0 2.192 2.192 0 00-1.736 1.039l-.821 1.316z"/>
  <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 12.75a4.5 4.5 0 11-9 0 4.5 4.5 0 019 0zM18.75 10.5h.008v.008h-.008V10.5z"/>
</svg>
  }
}
