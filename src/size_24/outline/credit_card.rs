use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%208%2E25h19%2E5M2%2E25%209h19%2E5m%2D16%2E5%205%2E25h6m%2D6%202%2E25h3m%2D3%2E75%203h15a2%2E25%202%2E25%200%20002%2E25%2D2%2E25V6%2E75A2%2E25%202%2E25%200%200019%2E5%204%2E5h%2D15a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25v10%2E5A2%2E25%202%2E25%200%20004%2E5%2019%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CreditCardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h3m-3.75 3h15a2.25 2.25 0 002.25-2.25V6.75A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25v10.5A2.25 2.25 0 004.5 19.5z"/>
</svg>
  }
}
