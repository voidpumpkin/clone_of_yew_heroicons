use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M13%2E887%203%2E182c%2E396%2E037%2E79%2E08%201%2E183%2E128C16%2E194%203%2E45%2017%204%2E414%2017%205%2E517V16%2E75A2%2E25%202%2E25%200%200114%2E75%2019h%2D9%2E5A2%2E25%202%2E25%200%20013%2016%2E75V5%2E517c0%2D1%2E103%2E806%2D2%2E068%201%2E93%2D2%2E207%2E393%2D%2E048%2E787%2D%2E09%201%2E183%2D%2E128A3%2E001%203%2E001%200%20019%201h2c1%2E373%200%202%2E531%2E923%202%2E887%202%2E182zM7%2E5%204A1%2E5%201%2E5%200%20019%202%2E5h2A1%2E5%201%2E5%200%200112%2E5%204v%2E5h%2D5V4z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ClipboardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M13.887 3.182c.396.037.79.08 1.183.128C16.194 3.45 17 4.414 17 5.517V16.75A2.25 2.25 0 0114.75 19h-9.5A2.25 2.25 0 013 16.75V5.517c0-1.103.806-2.068 1.93-2.207.393-.048.787-.09 1.183-.128A3.001 3.001 0 019 1h2c1.373 0 2.531.923 2.887 2.182zM7.5 4A1.5 1.5 0 019 2.5h2A1.5 1.5 0 0112.5 4v.5h-5V4z" clip-rule="evenodd"/>
</svg>
  }
}