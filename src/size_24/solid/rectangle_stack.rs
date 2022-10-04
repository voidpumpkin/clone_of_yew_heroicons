use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M5%2E566%204%2E657A4%2E505%204%2E505%200%20016%2E75%204%2E5h10%2E5c%2E41%200%20%2E806%2E055%201%2E183%2E157A3%203%200%200015%2E75%203h%2D7%2E5a3%203%200%2000%2D2%2E684%201%2E657zM2%2E25%2012a3%203%200%20013%2D3h13%2E5a3%203%200%20013%203v6a3%203%200%2001%2D3%203H5%2E25a3%203%200%2001%2D3%2D3v%2D6zM5%2E25%207%2E5c%2D%2E41%200%2D%2E806%2E055%2D1%2E184%2E157A3%203%200%20016%2E75%206h10%2E5a3%203%200%20012%2E683%201%2E657A4%2E505%204%2E505%200%200018%2E75%207%2E5H5%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RectangleStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M5.566 4.657A4.505 4.505 0 016.75 4.5h10.5c.41 0 .806.055 1.183.157A3 3 0 0015.75 3h-7.5a3 3 0 00-2.684 1.657zM2.25 12a3 3 0 013-3h13.5a3 3 0 013 3v6a3 3 0 01-3 3H5.25a3 3 0 01-3-3v-6zM5.25 7.5c-.41 0-.806.055-1.184.157A3 3 0 016.75 6h10.5a3 3 0 012.683 1.657A4.505 4.505 0 0018.75 7.5H5.25z"/>
</svg>
  }
}