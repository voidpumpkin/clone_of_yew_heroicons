use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M5%2E127%203%2E502L5%2E25%203%2E5h9%2E5c%2E041%200%20%2E082%200%20%2E123%2E002A2%2E251%202%2E251%200%200012%2E75%202h%2D5%2E5a2%2E25%202%2E25%200%2000%2D2%2E123%201%2E502zM1%2010%2E25A2%2E25%202%2E25%200%20013%2E25%208h13%2E5A2%2E25%202%2E25%200%200119%2010%2E25v5%2E5A2%2E25%202%2E25%200%200116%2E75%2018H3%2E25A2%2E25%202%2E25%200%20011%2015%2E75v%2D5%2E5zM3%2E25%206%2E5c%2D%2E04%200%2D%2E082%200%2D%2E123%2E002A2%2E25%202%2E25%200%20015%2E25%205h9%2E5c%2E98%200%201%2E814%2E627%202%2E123%201%2E502a3%2E819%203%2E819%200%2000%2D%2E123%2D%2E002H3%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RectangleStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M5.127 3.502L5.25 3.5h9.5c.041 0 .082 0 .123.002A2.251 2.251 0 0012.75 2h-5.5a2.25 2.25 0 00-2.123 1.502zM1 10.25A2.25 2.25 0 013.25 8h13.5A2.25 2.25 0 0119 10.25v5.5A2.25 2.25 0 0116.75 18H3.25A2.25 2.25 0 011 15.75v-5.5zM3.25 6.5c-.04 0-.082 0-.123.002A2.25 2.25 0 015.25 5h9.5c.98 0 1.814.627 2.123 1.502a3.819 3.819 0 00-.123-.002H3.25z"/>
</svg>
  }
}
