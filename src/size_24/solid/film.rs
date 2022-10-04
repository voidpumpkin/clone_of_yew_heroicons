use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%2E5%205%2E625c0%2D1%2E036%2E84%2D1%2E875%201%2E875%2D1%2E875h17%2E25c1%2E035%200%201%2E875%2E84%201%2E875%201%2E875v12%2E75c0%201%2E035%2D%2E84%201%2E875%2D1%2E875%201%2E875H3%2E375A1%2E875%201%2E875%200%20011%2E5%2018%2E375V5%2E625zm1%2E5%200v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375h1%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D1%2E5A%2E375%2E375%200%20003%205%2E625zm16%2E125%2D%2E375a%2E375%2E375%200%2000%2D%2E375%2E375v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375h1%2E5A%2E375%2E375%200%200021%207%2E125v%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D1%2E5zM21%209%2E375A%2E375%2E375%200%200020%2E625%209h%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2E375v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375h1%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D1%2E5zm0%203%2E75a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2E375v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375h1%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D1%2E5zm0%203%2E75a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2E375v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375h1%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D1%2E5zM4%2E875%2018%2E75a%2E375%2E375%200%2000%2E375%2D%2E375v%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2E375v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375h1%2E5zM3%2E375%2015h1%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D1%2E5a%2E375%2E375%200%2000%2D%2E375%2E375v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375zm0%2D3%2E75h1%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D1%2E5A%2E375%2E375%200%20004%2E875%209h%2D1%2E5A%2E375%2E375%200%20003%209%2E375v1%2E5c0%20%2E207%2E168%2E375%2E375%2E375zm4%2E125%200a%2E75%2E75%200%20000%201%2E5h9a%2E75%2E75%200%20000%2D1%2E5h%2D9z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FilmIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1.5 5.625c0-1.036.84-1.875 1.875-1.875h17.25c1.035 0 1.875.84 1.875 1.875v12.75c0 1.035-.84 1.875-1.875 1.875H3.375A1.875 1.875 0 011.5 18.375V5.625zm1.5 0v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5a.375.375 0 00-.375-.375h-1.5A.375.375 0 003 5.625zm16.125-.375a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5A.375.375 0 0021 7.125v-1.5a.375.375 0 00-.375-.375h-1.5zM21 9.375A.375.375 0 0020.625 9h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5zm0 3.75a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5zm0 3.75a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5a.375.375 0 00.375-.375v-1.5zM4.875 18.75a.375.375 0 00.375-.375v-1.5a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375h1.5zM3.375 15h1.5a.375.375 0 00.375-.375v-1.5a.375.375 0 00-.375-.375h-1.5a.375.375 0 00-.375.375v1.5c0 .207.168.375.375.375zm0-3.75h1.5a.375.375 0 00.375-.375v-1.5A.375.375 0 004.875 9h-1.5A.375.375 0 003 9.375v1.5c0 .207.168.375.375.375zm4.125 0a.75.75 0 000 1.5h9a.75.75 0 000-1.5h-9z" clip-rule="evenodd"/>
</svg>
  }
}