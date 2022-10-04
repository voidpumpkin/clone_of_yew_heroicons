use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M12%2E378%201%2E602a%2E75%2E75%200%2000%2D%2E756%200L3%206%2E632l9%205%2E25%209%2D5%2E25%2D8%2E622%2D5%2E03zM21%2E75%207%2E93l%2D9%205%2E25v9l8%2E628%2D5%2E032a%2E75%2E75%200%2000%2E372%2D%2E648V7%2E93zM11%2E25%2022%2E18v%2D9l%2D9%2D5%2E25v8%2E57a%2E75%2E75%200%2000%2E372%2E648l8%2E628%205%2E033z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CubeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M12.378 1.602a.75.75 0 00-.756 0L3 6.632l9 5.25 9-5.25-8.622-5.03zM21.75 7.93l-9 5.25v9l8.628-5.032a.75.75 0 00.372-.648V7.93zM11.25 22.18v-9l-9-5.25v8.57a.75.75 0 00.372.648l8.628 5.033z"/>
</svg>
  }
}
