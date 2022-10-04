use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M19%2E952%201%2E651a%2E75%2E75%200%2001%2E298%2E599V16%2E303a3%203%200%2001%2D2%2E176%202%2E884l%2D1%2E32%2E377a2%2E553%202%2E553%200%2011%2D1%2E403%2D4%2E909l2%2E311%2D%2E66a1%2E5%201%2E5%200%20001%2E088%2D1%2E442V6%2E994l%2D9%202%2E572v9%2E737a3%203%200%2001%2D2%2E176%202%2E884l%2D1%2E32%2E377a2%2E553%202%2E553%200%2011%2D1%2E402%2D4%2E909l2%2E31%2D%2E66a1%2E5%201%2E5%200%20001%2E088%2D1%2E442V9%2E017%205%2E25a%2E75%2E75%200%2001%2E544%2D%2E721l10%2E5%2D3a%2E75%2E75%200%2001%2E658%2E122z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MusicalNoteIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M19.952 1.651a.75.75 0 01.298.599V16.303a3 3 0 01-2.176 2.884l-1.32.377a2.553 2.553 0 11-1.403-4.909l2.311-.66a1.5 1.5 0 001.088-1.442V6.994l-9 2.572v9.737a3 3 0 01-2.176 2.884l-1.32.377a2.553 2.553 0 11-1.402-4.909l2.31-.66a1.5 1.5 0 001.088-1.442V9.017 5.25a.75.75 0 01.544-.721l10.5-3a.75.75 0 01.658.122z" clip-rule="evenodd"/>
</svg>
  }
}
