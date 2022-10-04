use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2E182%2015%2E182a4%2E5%204%2E5%200%2001%2D6%2E364%200M21%2012a9%209%200%2011%2D18%200%209%209%200%200118%200zM9%2E75%209%2E75c0%20%2E414%2D%2E168%2E75%2D%2E375%2E75S9%2010%2E164%209%209%2E75%209%2E168%209%209%2E375%209s%2E375%2E336%2E375%2E75zm%2D%2E375%200h%2E008v%2E015h%2D%2E008V9%2E75zm5%2E625%200c0%20%2E414%2D%2E168%2E75%2D%2E375%2E75s%2D%2E375%2D%2E336%2D%2E375%2D%2E75%2E168%2D%2E75%2E375%2D%2E75%2E375%2E336%2E375%2E75zm%2D%2E375%200h%2E008v%2E015h%2D%2E008V9%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FaceSmileIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.182 15.182a4.5 4.5 0 01-6.364 0M21 12a9 9 0 11-18 0 9 9 0 0118 0zM9.75 9.75c0 .414-.168.75-.375.75S9 10.164 9 9.75 9.168 9 9.375 9s.375.336.375.75zm-.375 0h.008v.015h-.008V9.75zm5.625 0c0 .414-.168.75-.375.75s-.375-.336-.375-.75.168-.75.375-.75.375.336.375.75zm-.375 0h.008v.015h-.008V9.75z"/>
</svg>
  }
}
