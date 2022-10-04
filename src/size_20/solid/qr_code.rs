use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E75%202A1%2E75%201%2E75%200%20002%203%2E75v3%2E5C2%208%2E216%202%2E784%209%203%2E75%209h3%2E5A1%2E75%201%2E75%200%20009%207%2E25v%2D3%2E5A1%2E75%201%2E75%200%20007%2E25%202h%2D3%2E5zM3%2E5%203%2E75a%2E25%2E25%200%2001%2E25%2D%2E25h3%2E5a%2E25%2E25%200%2001%2E25%2E25v3%2E5a%2E25%2E25%200%2001%2D%2E25%2E25h%2D3%2E5a%2E25%2E25%200%2001%2D%2E25%2D%2E25v%2D3%2E5zM3%2E75%2011A1%2E75%201%2E75%200%20002%2012%2E75v3%2E5c0%20%2E966%2E784%201%2E75%201%2E75%201%2E75h3%2E5A1%2E75%201%2E75%200%20009%2016%2E25v%2D3%2E5A1%2E75%201%2E75%200%20007%2E25%2011h%2D3%2E5zm%2D%2E25%201%2E75a%2E25%2E25%200%2001%2E25%2D%2E25h3%2E5a%2E25%2E25%200%2001%2E25%2E25v3%2E5a%2E25%2E25%200%2001%2D%2E25%2E25h%2D3%2E5a%2E25%2E25%200%2001%2D%2E25%2D%2E25v%2D3%2E5zm7%2E5%2D9c0%2D%2E966%2E784%2D1%2E75%201%2E75%2D1%2E75h3%2E5c%2E966%200%201%2E75%2E784%201%2E75%201%2E75v3%2E5A1%2E75%201%2E75%200%200116%2E25%209h%2D3%2E5A1%2E75%201%2E75%200%200111%207%2E25v%2D3%2E5zm1%2E75%2D%2E25a%2E25%2E25%200%2000%2D%2E25%2E25v3%2E5c0%20%2E138%2E112%2E25%2E25%2E25h3%2E5a%2E25%2E25%200%2000%2E25%2D%2E25v%2D3%2E5a%2E25%2E25%200%2000%2D%2E25%2D%2E25h%2D3%2E5zm%2D7%2E26%201a1%201%200%2000%2D1%201v%2E01a1%201%200%20001%201h%2E01a1%201%200%20001%2D1V5%2E5a1%201%200%2000%2D1%2D1h%2D%2E01zm9%200a1%201%200%2000%2D1%201v%2E01a1%201%200%20001%201h%2E01a1%201%200%20001%2D1V5%2E5a1%201%200%2000%2D1%2D1h%2D%2E01zm%2D9%209a1%201%200%2000%2D1%201v%2E01a1%201%200%20001%201h%2E01a1%201%200%20001%2D1v%2D%2E01a1%201%200%2000%2D1%2D1h%2D%2E01zm9%200a1%201%200%2000%2D1%201v%2E01a1%201%200%20001%201h%2E01a1%201%200%20001%2D1v%2D%2E01a1%201%200%2000%2D1%2D1h%2D%2E01zm%2D3%2E5%2D1%2E5a1%201%200%20011%2D1H12a1%201%200%20011%201v%2E01a1%201%200%2001%2D1%201h%2D%2E01a1%201%200%2001%2D1%2D1V12zm6%2D1a1%201%200%2000%2D1%201v%2E01a1%201%200%20001%201H17a1%201%200%20001%2D1V12a1%201%200%2000%2D1%2D1h%2D%2E01zm%2D1%206a1%201%200%20011%2D1H17a1%201%200%20011%201v%2E01a1%201%200%2001%2D1%201h%2D%2E01a1%201%200%2001%2D1%2D1V17zm%2D4%2D1a1%201%200%2000%2D1%201v%2E01a1%201%200%20001%201H12a1%201%200%20001%2D1V17a1%201%200%2000%2D1%2D1h%2D%2E01z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn QrCodeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.75 2A1.75 1.75 0 002 3.75v3.5C2 8.216 2.784 9 3.75 9h3.5A1.75 1.75 0 009 7.25v-3.5A1.75 1.75 0 007.25 2h-3.5zM3.5 3.75a.25.25 0 01.25-.25h3.5a.25.25 0 01.25.25v3.5a.25.25 0 01-.25.25h-3.5a.25.25 0 01-.25-.25v-3.5zM3.75 11A1.75 1.75 0 002 12.75v3.5c0 .966.784 1.75 1.75 1.75h3.5A1.75 1.75 0 009 16.25v-3.5A1.75 1.75 0 007.25 11h-3.5zm-.25 1.75a.25.25 0 01.25-.25h3.5a.25.25 0 01.25.25v3.5a.25.25 0 01-.25.25h-3.5a.25.25 0 01-.25-.25v-3.5zm7.5-9c0-.966.784-1.75 1.75-1.75h3.5c.966 0 1.75.784 1.75 1.75v3.5A1.75 1.75 0 0116.25 9h-3.5A1.75 1.75 0 0111 7.25v-3.5zm1.75-.25a.25.25 0 00-.25.25v3.5c0 .138.112.25.25.25h3.5a.25.25 0 00.25-.25v-3.5a.25.25 0 00-.25-.25h-3.5zm-7.26 1a1 1 0 00-1 1v.01a1 1 0 001 1h.01a1 1 0 001-1V5.5a1 1 0 00-1-1h-.01zm9 0a1 1 0 00-1 1v.01a1 1 0 001 1h.01a1 1 0 001-1V5.5a1 1 0 00-1-1h-.01zm-9 9a1 1 0 00-1 1v.01a1 1 0 001 1h.01a1 1 0 001-1v-.01a1 1 0 00-1-1h-.01zm9 0a1 1 0 00-1 1v.01a1 1 0 001 1h.01a1 1 0 001-1v-.01a1 1 0 00-1-1h-.01zm-3.5-1.5a1 1 0 011-1H12a1 1 0 011 1v.01a1 1 0 01-1 1h-.01a1 1 0 01-1-1V12zm6-1a1 1 0 00-1 1v.01a1 1 0 001 1H17a1 1 0 001-1V12a1 1 0 00-1-1h-.01zm-1 6a1 1 0 011-1H17a1 1 0 011 1v.01a1 1 0 01-1 1h-.01a1 1 0 01-1-1V17zm-4-1a1 1 0 00-1 1v.01a1 1 0 001 1H12a1 1 0 001-1V17a1 1 0 00-1-1h-.01z" clip-rule="evenodd"/>
</svg>
  }
}