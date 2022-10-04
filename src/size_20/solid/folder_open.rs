use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E75%203A1%2E75%201%2E75%200%20003%204%2E75v2%2E752l%2E104%2D%2E002h13%2E792c%2E035%200%20%2E07%200%20%2E104%2E002V6%2E75A1%2E75%201%2E75%200%200015%2E25%205h%2D3%2E836a%2E25%2E25%200%2001%2D%2E177%2D%2E073L9%2E823%203%2E513A1%2E75%201%2E75%200%20008%2E586%203H4%2E75zM3%2E104%209a1%2E75%201%2E75%200%2000%2D1%2E673%202%2E265l1%2E385%204%2E5A1%2E75%201%2E75%200%20004%2E488%2017h11%2E023a1%2E75%201%2E75%200%20001%2E673%2D1%2E235l1%2E384%2D4%2E5A1%2E75%201%2E75%200%200016%2E896%209H3%2E104z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FolderOpenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M4.75 3A1.75 1.75 0 003 4.75v2.752l.104-.002h13.792c.035 0 .07 0 .104.002V6.75A1.75 1.75 0 0015.25 5h-3.836a.25.25 0 01-.177-.073L9.823 3.513A1.75 1.75 0 008.586 3H4.75zM3.104 9a1.75 1.75 0 00-1.673 2.265l1.385 4.5A1.75 1.75 0 004.488 17h11.023a1.75 1.75 0 001.673-1.235l1.384-4.5A1.75 1.75 0 0016.896 9H3.104z"/>
</svg>
  }
}
