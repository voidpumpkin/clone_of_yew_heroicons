use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%202%2E25a%2E75%2E75%200%2001%2E75%2E75v9a%2E75%2E75%200%2001%2D1%2E5%200V3a%2E75%2E75%200%2001%2E75%2D%2E75zM6%2E166%205%2E106a%2E75%2E75%200%20010%201%2E06%208%2E25%208%2E25%200%201011%2E668%200%20%2E75%2E75%200%20111%2E06%2D1%2E06c3%2E808%203%2E807%203%2E808%209%2E98%200%2013%2E788%2D3%2E807%203%2E808%2D9%2E98%203%2E808%2D13%2E788%200%2D3%2E808%2D3%2E807%2D3%2E808%2D9%2E98%200%2D13%2E788a%2E75%2E75%200%20011%2E06%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PowerIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 2.25a.75.75 0 01.75.75v9a.75.75 0 01-1.5 0V3a.75.75 0 01.75-.75zM6.166 5.106a.75.75 0 010 1.06 8.25 8.25 0 1011.668 0 .75.75 0 111.06-1.06c3.808 3.807 3.808 9.98 0 13.788-3.807 3.808-9.98 3.808-13.788 0-3.808-3.807-3.808-9.98 0-13.788a.75.75 0 011.06 0z" clip-rule="evenodd"/>
</svg>
  }
}