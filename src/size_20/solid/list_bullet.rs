use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%204%2E75A%2E75%2E75%200%20016%2E75%204h10%2E5a%2E75%2E75%200%20010%201%2E5H6%2E75A%2E75%2E75%200%20016%204%2E75zM6%2010a%2E75%2E75%200%2001%2E75%2D%2E75h10%2E5a%2E75%2E75%200%20010%201%2E5H6%2E75A%2E75%2E75%200%20016%2010zm0%205%2E25a%2E75%2E75%200%2001%2E75%2D%2E75h10%2E5a%2E75%2E75%200%20010%201%2E5H6%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75zM1%2E99%204%2E75a1%201%200%20011%2D1H3a1%201%200%20011%201v%2E01a1%201%200%2001%2D1%201h%2D%2E01a1%201%200%2001%2D1%2D1v%2D%2E01zM1%2E99%2015%2E25a1%201%200%20011%2D1H3a1%201%200%20011%201v%2E01a1%201%200%2001%2D1%201h%2D%2E01a1%201%200%2001%2D1%2D1v%2D%2E01zM1%2E99%2010a1%201%200%20011%2D1H3a1%201%200%20011%201v%2E01a1%201%200%2001%2D1%201h%2D%2E01a1%201%200%2001%2D1%2D1V10z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ListBulletIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6 4.75A.75.75 0 016.75 4h10.5a.75.75 0 010 1.5H6.75A.75.75 0 016 4.75zM6 10a.75.75 0 01.75-.75h10.5a.75.75 0 010 1.5H6.75A.75.75 0 016 10zm0 5.25a.75.75 0 01.75-.75h10.5a.75.75 0 010 1.5H6.75a.75.75 0 01-.75-.75zM1.99 4.75a1 1 0 011-1H3a1 1 0 011 1v.01a1 1 0 01-1 1h-.01a1 1 0 01-1-1v-.01zM1.99 15.25a1 1 0 011-1H3a1 1 0 011 1v.01a1 1 0 01-1 1h-.01a1 1 0 01-1-1v-.01zM1.99 10a1 1 0 011-1H3a1 1 0 011 1v.01a1 1 0 01-1 1h-.01a1 1 0 01-1-1V10z" clip-rule="evenodd"/>
</svg>
  }
}