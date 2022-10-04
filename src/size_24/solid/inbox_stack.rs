use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%2E5%209%2E832v1%2E793c0%201%2E036%2E84%201%2E875%201%2E875%201%2E875h17%2E25c1%2E035%200%201%2E875%2D%2E84%201%2E875%2D1%2E875V9%2E832a3%203%200%2000%2D%2E722%2D1%2E952l%2D3%2E285%2D3%2E832A3%203%200%200016%2E215%203h%2D8%2E43a3%203%200%2000%2D2%2E278%201%2E048L2%2E222%207%2E88A3%203%200%20001%2E5%209%2E832zM7%2E785%204%2E5a1%2E5%201%2E5%200%2000%2D1%2E139%2E524L3%2E881%208%2E25h3%2E165a3%203%200%20012%2E496%201%2E336l%2E164%2E246a1%2E5%201%2E5%200%20001%2E248%2E668h2%2E092a1%2E5%201%2E5%200%20001%2E248%2D%2E668l%2E164%2D%2E246a3%203%200%20012%2E496%2D1%2E336h3%2E165l%2D2%2E765%2D3%2E226a1%2E5%201%2E5%200%2000%2D1%2E139%2D%2E524h%2D8%2E43z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M2%2E813%2015c%2D%2E725%200%2D1%2E313%2E588%2D1%2E313%201%2E313V18a3%203%200%20003%203h15a3%203%200%20003%2D3v%2D1%2E688c0%2D%2E724%2D%2E588%2D1%2E312%2D1%2E313%2D1%2E312h%2D4%2E233a3%203%200%2000%2D2%2E496%201%2E336l%2D%2E164%2E246a1%2E5%201%2E5%200%2001%2D1%2E248%2E668h%2D2%2E092a1%2E5%201%2E5%200%2001%2D1%2E248%2D%2E668l%2D%2E164%2D%2E246A3%203%200%20007%2E046%2015H2%2E812z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn InboxStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1.5 9.832v1.793c0 1.036.84 1.875 1.875 1.875h17.25c1.035 0 1.875-.84 1.875-1.875V9.832a3 3 0 00-.722-1.952l-3.285-3.832A3 3 0 0016.215 3h-8.43a3 3 0 00-2.278 1.048L2.222 7.88A3 3 0 001.5 9.832zM7.785 4.5a1.5 1.5 0 00-1.139.524L3.881 8.25h3.165a3 3 0 012.496 1.336l.164.246a1.5 1.5 0 001.248.668h2.092a1.5 1.5 0 001.248-.668l.164-.246a3 3 0 012.496-1.336h3.165l-2.765-3.226a1.5 1.5 0 00-1.139-.524h-8.43z" clip-rule="evenodd"/>
  <path d="M2.813 15c-.725 0-1.313.588-1.313 1.313V18a3 3 0 003 3h15a3 3 0 003-3v-1.688c0-.724-.588-1.312-1.313-1.312h-4.233a3 3 0 00-2.496 1.336l-.164.246a1.5 1.5 0 01-1.248.668h-2.092a1.5 1.5 0 01-1.248-.668l-.164-.246A3 3 0 007.046 15H2.812z"/>
</svg>
  }
}