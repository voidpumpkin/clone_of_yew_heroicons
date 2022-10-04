use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M15%2E5%202A1%2E5%201%2E5%200%200014%203%2E5v13a1%2E5%201%2E5%200%20001%2E5%201%2E5h1a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D13A1%2E5%201%2E5%200%200016%2E5%202h%2D1zM9%2E5%206A1%2E5%201%2E5%200%20008%207%2E5v9A1%2E5%201%2E5%200%20009%2E5%2018h1a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D9A1%2E5%201%2E5%200%200010%2E5%206h%2D1zM3%2E5%2010A1%2E5%201%2E5%200%20002%2011%2E5v5A1%2E5%201%2E5%200%20003%2E5%2018h1A1%2E5%201%2E5%200%20006%2016%2E5v%2D5A1%2E5%201%2E5%200%20004%2E5%2010h%2D1z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChartBarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M15.5 2A1.5 1.5 0 0014 3.5v13a1.5 1.5 0 001.5 1.5h1a1.5 1.5 0 001.5-1.5v-13A1.5 1.5 0 0016.5 2h-1zM9.5 6A1.5 1.5 0 008 7.5v9A1.5 1.5 0 009.5 18h1a1.5 1.5 0 001.5-1.5v-9A1.5 1.5 0 0010.5 6h-1zM3.5 10A1.5 1.5 0 002 11.5v5A1.5 1.5 0 003.5 18h1A1.5 1.5 0 006 16.5v-5A1.5 1.5 0 004.5 10h-1z"/>
</svg>
  }
}
