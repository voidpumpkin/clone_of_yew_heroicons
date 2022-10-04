use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E25%203A2%2E25%202%2E25%200%20001%205%2E25v9%2E5A2%2E25%202%2E25%200%20003%2E25%2017h13%2E5A2%2E25%202%2E25%200%200019%2014%2E75v%2D9%2E5A2%2E25%202%2E25%200%200016%2E75%203H3%2E25zm%2E943%208%2E752a%2E75%2E75%200%2001%2E055%2D1%2E06L6%2E128%209l%2D1%2E88%2D1%2E693a%2E75%2E75%200%20111%2E004%2D1%2E114l2%2E5%202%2E25a%2E75%2E75%200%20010%201%2E114l%2D2%2E5%202%2E25a%2E75%2E75%200%2001%2D1%2E06%2D%2E055zM9%2E75%2010%2E25a%2E75%2E75%200%20000%201%2E5h2%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D2%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CommandLineIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.25 3A2.25 2.25 0 001 5.25v9.5A2.25 2.25 0 003.25 17h13.5A2.25 2.25 0 0019 14.75v-9.5A2.25 2.25 0 0016.75 3H3.25zm.943 8.752a.75.75 0 01.055-1.06L6.128 9l-1.88-1.693a.75.75 0 111.004-1.114l2.5 2.25a.75.75 0 010 1.114l-2.5 2.25a.75.75 0 01-1.06-.055zM9.75 10.25a.75.75 0 000 1.5h2.5a.75.75 0 000-1.5h-2.5z" clip-rule="evenodd"/>
</svg>
  }
}