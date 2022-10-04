use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%2E963%202%2E286a%2E75%2E75%200%2000%2D1%2E071%2D%2E136%209%2E742%209%2E742%200%2000%2D3%2E539%206%2E177A7%2E547%207%2E547%200%20016%2E648%206%2E61a%2E75%2E75%200%2000%2D1%2E152%2D%2E082A9%209%200%201015%2E68%204%2E534a7%2E46%207%2E46%200%2001%2D2%2E717%2D2%2E248zM15%2E75%2014%2E25a3%2E75%203%2E75%200%2011%2D7%2E313%2D1%2E172c%2E628%2E465%201%2E35%2E81%202%2E133%201a5%2E99%205%2E99%200%20011%2E925%2D3%2E545%203%2E75%203%2E75%200%20013%2E255%203%2E717z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FireIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12.963 2.286a.75.75 0 00-1.071-.136 9.742 9.742 0 00-3.539 6.177A7.547 7.547 0 016.648 6.61a.75.75 0 00-1.152-.082A9 9 0 1015.68 4.534a7.46 7.46 0 01-2.717-2.248zM15.75 14.25a3.75 3.75 0 11-7.313-1.172c.628.465 1.35.81 2.133 1a5.99 5.99 0 011.925-3.545 3.75 3.75 0 013.255 3.717z" clip-rule="evenodd"/>
</svg>
  }
}
