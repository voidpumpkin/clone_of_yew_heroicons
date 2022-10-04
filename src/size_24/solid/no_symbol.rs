use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%2E72%205%2E66l11%2E62%2011%2E62A8%2E25%208%2E25%200%20006%2E72%205%2E66zm10%2E56%2012%2E68L5%2E66%206%2E72a8%2E25%208%2E25%200%200011%2E62%2011%2E62zM5%2E105%205%2E106c3%2E807%2D3%2E808%209%2E98%2D3%2E808%2013%2E788%200%203%2E808%203%2E807%203%2E808%209%2E98%200%2013%2E788%2D3%2E807%203%2E808%2D9%2E98%203%2E808%2D13%2E788%200%2D3%2E808%2D3%2E807%2D3%2E808%2D9%2E98%200%2D13%2E788z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn NoSymbolIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6.72 5.66l11.62 11.62A8.25 8.25 0 006.72 5.66zm10.56 12.68L5.66 6.72a8.25 8.25 0 0011.62 11.62zM5.105 5.106c3.807-3.808 9.98-3.808 13.788 0 3.808 3.807 3.808 9.98 0 13.788-3.807 3.808-9.98 3.808-13.788 0-3.808-3.807-3.808-9.98 0-13.788z" clip-rule="evenodd"/>
</svg>
  }
}
