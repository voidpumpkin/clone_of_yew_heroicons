use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%2E516%202%2E17a%2E75%2E75%200%2000%2D1%2E032%200%2011%2E209%2011%2E209%200%2001%2D7%2E877%203%2E08%2E75%2E75%200%2000%2D%2E722%2E515A12%2E74%2012%2E74%200%20002%2E25%209%2E75c0%205%2E942%204%2E064%2010%2E933%209%2E563%2012%2E348a%2E749%2E749%200%2000%2E374%200c5%2E499%2D1%2E415%209%2E563%2D6%2E406%209%2E563%2D12%2E348%200%2D1%2E39%2D%2E223%2D2%2E73%2D%2E635%2D3%2E985a%2E75%2E75%200%2000%2D%2E722%2D%2E516l%2D%2E143%2E001c%2D2%2E996%200%2D5%2E717%2D1%2E17%2D7%2E734%2D3%2E08zm3%2E094%208%2E016a%2E75%2E75%200%2010%2D1%2E22%2D%2E872l%2D3%2E236%204%2E53L9%2E53%2012%2E22a%2E75%2E75%200%2000%2D1%2E06%201%2E06l2%2E25%202%2E25a%2E75%2E75%200%20001%2E14%2D%2E094l3%2E75%2D5%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShieldCheckIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12.516 2.17a.75.75 0 00-1.032 0 11.209 11.209 0 01-7.877 3.08.75.75 0 00-.722.515A12.74 12.74 0 002.25 9.75c0 5.942 4.064 10.933 9.563 12.348a.749.749 0 00.374 0c5.499-1.415 9.563-6.406 9.563-12.348 0-1.39-.223-2.73-.635-3.985a.75.75 0 00-.722-.516l-.143.001c-2.996 0-5.717-1.17-7.734-3.08zm3.094 8.016a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
</svg>
  }
}
