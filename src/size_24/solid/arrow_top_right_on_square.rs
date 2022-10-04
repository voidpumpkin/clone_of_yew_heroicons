use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M15%2E75%202%2E25H21a%2E75%2E75%200%2001%2E75%2E75v5%2E25a%2E75%2E75%200%2001%2D1%2E5%200V4%2E81L8%2E03%2017%2E03a%2E75%2E75%200%2001%2D1%2E06%2D1%2E06L19%2E19%203%2E75h%2D3%2E44a%2E75%2E75%200%20010%2D1%2E5zm%2D10%2E5%204%2E5a1%2E5%201%2E5%200%2000%2D1%2E5%201%2E5v10%2E5a1%2E5%201%2E5%200%20001%2E5%201%2E5h10%2E5a1%2E5%201%2E5%200%20001%2E5%2D1%2E5V10%2E5a%2E75%2E75%200%20011%2E5%200v8%2E25a3%203%200%2001%2D3%203H5%2E25a3%203%200%2001%2D3%2D3V8%2E25a3%203%200%20013%2D3h8%2E25a%2E75%2E75%200%20010%201%2E5H5%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowTopRightOnSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M15.75 2.25H21a.75.75 0 01.75.75v5.25a.75.75 0 01-1.5 0V4.81L8.03 17.03a.75.75 0 01-1.06-1.06L19.19 3.75h-3.44a.75.75 0 010-1.5zm-10.5 4.5a1.5 1.5 0 00-1.5 1.5v10.5a1.5 1.5 0 001.5 1.5h10.5a1.5 1.5 0 001.5-1.5V10.5a.75.75 0 011.5 0v8.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V8.25a3 3 0 013-3h8.25a.75.75 0 010 1.5H5.25z" clip-rule="evenodd"/>
</svg>
  }
}