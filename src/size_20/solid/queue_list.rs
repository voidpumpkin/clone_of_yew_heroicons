use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M2%204%2E5A2%2E5%202%2E5%200%20014%2E5%202h11a2%2E5%202%2E5%200%20010%205h%2D11A2%2E5%202%2E5%200%20012%204%2E5zM2%2E75%209%2E083a%2E75%2E75%200%20000%201%2E5h14%2E5a%2E75%2E75%200%20000%2D1%2E5H2%2E75zM2%2E75%2012%2E663a%2E75%2E75%200%20000%201%2E5h14%2E5a%2E75%2E75%200%20000%2D1%2E5H2%2E75zM2%2E75%2016%2E25a%2E75%2E75%200%20000%201%2E5h14%2E5a%2E75%2E75%200%20100%2D1%2E5H2%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn QueueListIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M2 4.5A2.5 2.5 0 014.5 2h11a2.5 2.5 0 010 5h-11A2.5 2.5 0 012 4.5zM2.75 9.083a.75.75 0 000 1.5h14.5a.75.75 0 000-1.5H2.75zM2.75 12.663a.75.75 0 000 1.5h14.5a.75.75 0 000-1.5H2.75zM2.75 16.25a.75.75 0 000 1.5h14.5a.75.75 0 100-1.5H2.75z"/>
</svg>
  }
}