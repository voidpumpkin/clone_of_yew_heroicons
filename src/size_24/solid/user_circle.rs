use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M18%2E685%2019%2E097A9%2E723%209%2E723%200%200021%2E75%2012c0%2D5%2E385%2D4%2E365%2D9%2E75%2D9%2E75%2D9%2E75S2%2E25%206%2E615%202%2E25%2012a9%2E723%209%2E723%200%20003%2E065%207%2E097A9%2E716%209%2E716%200%200012%2021%2E75a9%2E716%209%2E716%200%20006%2E685%2D2%2E653zm%2D12%2E54%2D1%2E285A7%2E486%207%2E486%200%200112%2015a7%2E486%207%2E486%200%20015%2E855%202%2E812A8%2E224%208%2E224%200%200112%2020%2E25a8%2E224%208%2E224%200%2001%2D5%2E855%2D2%2E438zM15%2E75%209a3%2E75%203%2E75%200%2011%2D7%2E5%200%203%2E75%203%2E75%200%20017%2E5%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M18.685 19.097A9.723 9.723 0 0021.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 003.065 7.097A9.716 9.716 0 0012 21.75a9.716 9.716 0 006.685-2.653zm-12.54-1.285A7.486 7.486 0 0112 15a7.486 7.486 0 015.855 2.812A8.224 8.224 0 0112 20.25a8.224 8.224 0 01-5.855-2.438zM15.75 9a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" clip-rule="evenodd"/>
</svg>
  }
}