use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E25%202A2%2E25%202%2E25%200%20002%204%2E25v2a%2E75%2E75%200%20001%2E5%200v%2D2a%2E75%2E75%200%2001%2E75%2D%2E75h2a%2E75%2E75%200%20000%2D1%2E5h%2D2zM13%2E75%202a%2E75%2E75%200%20000%201%2E5h2a%2E75%2E75%200%2001%2E75%2E75v2a%2E75%2E75%200%20001%2E5%200v%2D2A2%2E25%202%2E25%200%200015%2E75%202h%2D2zM3%2E5%2013%2E75a%2E75%2E75%200%2000%2D1%2E5%200v2A2%2E25%202%2E25%200%20004%2E25%2018h2a%2E75%2E75%200%20000%2D1%2E5h%2D2a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D2zM18%2013%2E75a%2E75%2E75%200%2000%2D1%2E5%200v2a%2E75%2E75%200%2001%2D%2E75%2E75h%2D2a%2E75%2E75%200%20000%201%2E5h2A2%2E25%202%2E25%200%200018%2015%2E75v%2D2zM7%2010a3%203%200%20116%200%203%203%200%2001%2D6%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ViewfinderCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M4.25 2A2.25 2.25 0 002 4.25v2a.75.75 0 001.5 0v-2a.75.75 0 01.75-.75h2a.75.75 0 000-1.5h-2zM13.75 2a.75.75 0 000 1.5h2a.75.75 0 01.75.75v2a.75.75 0 001.5 0v-2A2.25 2.25 0 0015.75 2h-2zM3.5 13.75a.75.75 0 00-1.5 0v2A2.25 2.25 0 004.25 18h2a.75.75 0 000-1.5h-2a.75.75 0 01-.75-.75v-2zM18 13.75a.75.75 0 00-1.5 0v2a.75.75 0 01-.75.75h-2a.75.75 0 000 1.5h2A2.25 2.25 0 0018 15.75v-2zM7 10a3 3 0 116 0 3 3 0 01-6 0z"/>
</svg>
  }
}
