use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M18%2E97%203%2E659a2%2E25%202%2E25%200%2000%2D3%2E182%200l%2D10%2E94%2010%2E94a3%2E75%203%2E75%200%20105%2E304%205%2E303l7%2E693%2D7%2E693a%2E75%2E75%200%20011%2E06%201%2E06l%2D7%2E693%207%2E693a5%2E25%205%2E25%200%2011%2D7%2E424%2D7%2E424l10%2E939%2D10%2E94a3%2E75%203%2E75%200%20115%2E303%205%2E304L9%2E097%2018%2E835l%2D%2E008%2E008%2D%2E007%2E007%2D%2E002%2E002%2D%2E003%2E002A2%2E25%202%2E25%200%20015%2E91%2015%2E66l7%2E81%2D7%2E81a%2E75%2E75%200%20011%2E061%201%2E06l%2D7%2E81%207%2E81a%2E75%2E75%200%20001%2E054%201%2E068L18%2E97%206%2E84a2%2E25%202%2E25%200%20000%2D3%2E182z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PaperClipIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M18.97 3.659a2.25 2.25 0 00-3.182 0l-10.94 10.94a3.75 3.75 0 105.304 5.303l7.693-7.693a.75.75 0 011.06 1.06l-7.693 7.693a5.25 5.25 0 11-7.424-7.424l10.939-10.94a3.75 3.75 0 115.303 5.304L9.097 18.835l-.008.008-.007.007-.002.002-.003.002A2.25 2.25 0 015.91 15.66l7.81-7.81a.75.75 0 011.061 1.06l-7.81 7.81a.75.75 0 001.054 1.068L18.97 6.84a2.25 2.25 0 000-3.182z" clip-rule="evenodd"/>
</svg>
  }
}