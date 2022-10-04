use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M8%2E485%202%2E495c%2E673%2D1%2E167%202%2E357%2D1%2E167%203%2E03%200l6%2E28%2010%2E875c%2E673%201%2E167%2D%2E17%202%2E625%2D1%2E516%202%2E625H3%2E72c%2D1%2E347%200%2D2%2E189%2D1%2E458%2D1%2E515%2D2%2E625L8%2E485%202%2E495zM10%205a%2E75%2E75%200%2001%2E75%2E75v3%2E5a%2E75%2E75%200%2001%2D1%2E5%200v%2D3%2E5A%2E75%2E75%200%200110%205zm0%209a1%201%200%20100%2D2%201%201%200%20000%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ExclamationTriangleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
</svg>
  }
}