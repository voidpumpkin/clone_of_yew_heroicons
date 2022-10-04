use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M11%205a3%203%200%2011%2D6%200%203%203%200%20016%200zM2%2E046%2015%2E253c%2D%2E058%2E468%2E172%2E92%2E57%201%2E175A9%2E953%209%2E953%200%20008%2018c1%2E982%200%203%2E83%2D%2E578%205%2E384%2D1%2E573%2E398%2D%2E254%2E628%2D%2E707%2E57%2D1%2E175a6%2E001%206%2E001%200%2000%2D11%2E908%200zM12%2E75%207%2E75a%2E75%2E75%200%20000%201%2E5h5%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D5%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserMinusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M11 5a3 3 0 11-6 0 3 3 0 016 0zM2.046 15.253c-.058.468.172.92.57 1.175A9.953 9.953 0 008 18c1.982 0 3.83-.578 5.384-1.573.398-.254.628-.707.57-1.175a6.001 6.001 0 00-11.908 0zM12.75 7.75a.75.75 0 000 1.5h5.5a.75.75 0 000-1.5h-5.5z"/>
</svg>
  }
}
