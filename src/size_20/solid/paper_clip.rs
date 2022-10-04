use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M15%2E621%204%2E379a3%203%200%2000%2D4%2E242%200l%2D7%207a3%203%200%20004%2E241%204%2E243h%2E001l%2E497%2D%2E5a%2E75%2E75%200%20011%2E064%201%2E057l%2D%2E498%2E501%2D%2E002%2E002a4%2E5%204%2E5%200%2001%2D6%2E364%2D6%2E364l7%2D7a4%2E5%204%2E5%200%20016%2E368%206%2E36l%2D3%2E455%203%2E553A2%2E625%202%2E625%200%20119%2E52%209%2E52l3%2E45%2D3%2E451a%2E75%2E75%200%20111%2E061%201%2E06l%2D3%2E45%203%2E451a1%2E125%201%2E125%200%20001%2E587%201%2E595l3%2E454%2D3%2E553a3%203%200%20000%2D4%2E242z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PaperClipIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M15.621 4.379a3 3 0 00-4.242 0l-7 7a3 3 0 004.241 4.243h.001l.497-.5a.75.75 0 011.064 1.057l-.498.501-.002.002a4.5 4.5 0 01-6.364-6.364l7-7a4.5 4.5 0 016.368 6.36l-3.455 3.553A2.625 2.625 0 119.52 9.52l3.45-3.451a.75.75 0 111.061 1.06l-3.45 3.451a1.125 1.125 0 001.587 1.595l3.454-3.553a3 3 0 000-4.242z" clip-rule="evenodd"/>
</svg>
  }
}
