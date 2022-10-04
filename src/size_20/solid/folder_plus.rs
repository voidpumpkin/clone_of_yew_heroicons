use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E75%203A1%2E75%201%2E75%200%20002%204%2E75v10%2E5c0%20%2E966%2E784%201%2E75%201%2E75%201%2E75h12%2E5A1%2E75%201%2E75%200%200018%2015%2E25v%2D8%2E5A1%2E75%201%2E75%200%200016%2E25%205h%2D4%2E836a%2E25%2E25%200%2001%2D%2E177%2D%2E073L9%2E823%203%2E513A1%2E75%201%2E75%200%20008%2E586%203H3%2E75zM10%208a%2E75%2E75%200%2001%2E75%2E75v1%2E5h1%2E5a%2E75%2E75%200%20010%201%2E5h%2D1%2E5v1%2E5a%2E75%2E75%200%2001%2D1%2E5%200v%2D1%2E5h%2D1%2E5a%2E75%2E75%200%20010%2D1%2E5h1%2E5v%2D1%2E5A%2E75%2E75%200%200110%208z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FolderPlusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.75 3A1.75 1.75 0 002 4.75v10.5c0 .966.784 1.75 1.75 1.75h12.5A1.75 1.75 0 0018 15.25v-8.5A1.75 1.75 0 0016.25 5h-4.836a.25.25 0 01-.177-.073L9.823 3.513A1.75 1.75 0 008.586 3H3.75zM10 8a.75.75 0 01.75.75v1.5h1.5a.75.75 0 010 1.5h-1.5v1.5a.75.75 0 01-1.5 0v-1.5h-1.5a.75.75 0 010-1.5h1.5v-1.5A.75.75 0 0110 8z" clip-rule="evenodd"/>
</svg>
  }
}
