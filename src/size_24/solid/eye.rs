use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M12%2015a3%203%200%20100%2D6%203%203%200%20000%206z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%2E323%2011%2E447C2%2E811%206%2E976%207%2E028%203%2E75%2012%2E001%203%2E75c4%2E97%200%209%2E185%203%2E223%2010%2E675%207%2E69%2E12%2E362%2E12%2E752%200%201%2E113%2D1%2E487%204%2E471%2D5%2E705%207%2E697%2D10%2E677%207%2E697%2D4%2E97%200%2D9%2E186%2D3%2E223%2D10%2E675%2D7%2E69a1%2E762%201%2E762%200%20010%2D1%2E113zM17%2E25%2012a5%2E25%205%2E25%200%2011%2D10%2E5%200%205%2E25%205%2E25%200%200110%2E5%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EyeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M12 15a3 3 0 100-6 3 3 0 000 6z"/>
  <path fill-rule="evenodd" d="M1.323 11.447C2.811 6.976 7.028 3.75 12.001 3.75c4.97 0 9.185 3.223 10.675 7.69.12.362.12.752 0 1.113-1.487 4.471-5.705 7.697-10.677 7.697-4.97 0-9.186-3.223-10.675-7.69a1.762 1.762 0 010-1.113zM17.25 12a5.25 5.25 0 11-10.5 0 5.25 5.25 0 0110.5 0z" clip-rule="evenodd"/>
</svg>
  }
}