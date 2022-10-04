use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%204%2E875C3%203%2E839%203%2E84%203%204%2E875%203h4%2E5c1%2E036%200%201%2E875%2E84%201%2E875%201%2E875v4%2E5c0%201%2E036%2D%2E84%201%2E875%2D1%2E875%201%2E875h%2D4%2E5A1%2E875%201%2E875%200%20013%209%2E375v%2D4%2E5zM4%2E875%204%2E5a%2E375%2E375%200%2000%2D%2E375%2E375v4%2E5c0%20%2E207%2E168%2E375%2E375%2E375h4%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D4%2E5a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D4%2E5zm7%2E875%2E375c0%2D1%2E036%2E84%2D1%2E875%201%2E875%2D1%2E875h4%2E5C20%2E16%203%2021%203%2E84%2021%204%2E875v4%2E5c0%201%2E036%2D%2E84%201%2E875%2D1%2E875%201%2E875h%2D4%2E5a1%2E875%201%2E875%200%2001%2D1%2E875%2D1%2E875v%2D4%2E5zm1%2E875%2D%2E375a%2E375%2E375%200%2000%2D%2E375%2E375v4%2E5c0%20%2E207%2E168%2E375%2E375%2E375h4%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D4%2E5a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D4%2E5zM6%206%2E75A%2E75%2E75%200%20016%2E75%206h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75A%2E75%2E75%200%20016%207%2E5v%2D%2E75zm9%2E75%200A%2E75%2E75%200%200116%2E5%206h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75zM3%2014%2E625c0%2D1%2E036%2E84%2D1%2E875%201%2E875%2D1%2E875h4%2E5c1%2E036%200%201%2E875%2E84%201%2E875%201%2E875v4%2E5c0%201%2E035%2D%2E84%201%2E875%2D1%2E875%201%2E875h%2D4%2E5A1%2E875%201%2E875%200%20013%2019%2E125v%2D4%2E5zm1%2E875%2D%2E375a%2E375%2E375%200%2000%2D%2E375%2E375v4%2E5c0%20%2E207%2E168%2E375%2E375%2E375h4%2E5a%2E375%2E375%200%2000%2E375%2D%2E375v%2D4%2E5a%2E375%2E375%200%2000%2D%2E375%2D%2E375h%2D4%2E5zm7%2E875%2D%2E75a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75zm6%200a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75zM6%2016%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75zm9%2E75%200a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75zm%2D3%203a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75zm6%200a%2E75%2E75%200%2001%2E75%2D%2E75h%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn QrCodeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 4.875C3 3.839 3.84 3 4.875 3h4.5c1.036 0 1.875.84 1.875 1.875v4.5c0 1.036-.84 1.875-1.875 1.875h-4.5A1.875 1.875 0 013 9.375v-4.5zM4.875 4.5a.375.375 0 00-.375.375v4.5c0 .207.168.375.375.375h4.5a.375.375 0 00.375-.375v-4.5a.375.375 0 00-.375-.375h-4.5zm7.875.375c0-1.036.84-1.875 1.875-1.875h4.5C20.16 3 21 3.84 21 4.875v4.5c0 1.036-.84 1.875-1.875 1.875h-4.5a1.875 1.875 0 01-1.875-1.875v-4.5zm1.875-.375a.375.375 0 00-.375.375v4.5c0 .207.168.375.375.375h4.5a.375.375 0 00.375-.375v-4.5a.375.375 0 00-.375-.375h-4.5zM6 6.75A.75.75 0 016.75 6h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75A.75.75 0 016 7.5v-.75zm9.75 0A.75.75 0 0116.5 6h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75zM3 14.625c0-1.036.84-1.875 1.875-1.875h4.5c1.036 0 1.875.84 1.875 1.875v4.5c0 1.035-.84 1.875-1.875 1.875h-4.5A1.875 1.875 0 013 19.125v-4.5zm1.875-.375a.375.375 0 00-.375.375v4.5c0 .207.168.375.375.375h4.5a.375.375 0 00.375-.375v-4.5a.375.375 0 00-.375-.375h-4.5zm7.875-.75a.75.75 0 01.75-.75h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75zm6 0a.75.75 0 01.75-.75h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75zM6 16.5a.75.75 0 01.75-.75h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75zm9.75 0a.75.75 0 01.75-.75h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75zm-3 3a.75.75 0 01.75-.75h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75zm6 0a.75.75 0 01.75-.75h.75a.75.75 0 01.75.75v.75a.75.75 0 01-.75.75h-.75a.75.75 0 01-.75-.75v-.75z" clip-rule="evenodd"/>
</svg>
  }
}