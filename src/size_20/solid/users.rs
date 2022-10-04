use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M7%208a3%203%200%20100%2D6%203%203%200%20000%206zM14%2E5%209a2%2E5%202%2E5%200%20100%2D5%202%2E5%202%2E5%200%20000%205zM1%2E615%2016%2E428a1%2E224%201%2E224%200%2001%2D%2E569%2D1%2E175%206%2E002%206%2E002%200%200111%2E908%200c%2E058%2E467%2D%2E172%2E92%2D%2E57%201%2E174A9%2E953%209%2E953%200%20017%2018a9%2E953%209%2E953%200%2001%2D5%2E385%2D1%2E572zM14%2E5%2016h%2D%2E106c%2E07%2D%2E297%2E088%2D%2E611%2E048%2D%2E933a7%2E47%207%2E47%200%2000%2D1%2E588%2D3%2E755%204%2E502%204%2E502%200%20015%2E874%202%2E636%2E818%2E818%200%2001%2D%2E36%2E98A7%2E465%207%2E465%200%200114%2E5%2016z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UsersIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M7 8a3 3 0 100-6 3 3 0 000 6zM14.5 9a2.5 2.5 0 100-5 2.5 2.5 0 000 5zM1.615 16.428a1.224 1.224 0 01-.569-1.175 6.002 6.002 0 0111.908 0c.058.467-.172.92-.57 1.174A9.953 9.953 0 017 18a9.953 9.953 0 01-5.385-1.572zM14.5 16h-.106c.07-.297.088-.611.048-.933a7.47 7.47 0 00-1.588-3.755 4.502 4.502 0 015.874 2.636.818.818 0 01-.36.98A7.465 7.465 0 0114.5 16z"/>
</svg>
  }
}