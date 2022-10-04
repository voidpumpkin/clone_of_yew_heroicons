use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M18%2E375%2012%2E739l%2D7%2E693%207%2E693a4%2E5%204%2E5%200%2001%2D6%2E364%2D6%2E364l10%2E94%2D10%2E94A3%203%200%201119%2E5%207%2E372L8%2E552%2018%2E32m%2E009%2D%2E01l%2D%2E01%2E01m5%2E699%2D9%2E941l%2D7%2E81%207%2E81a1%2E5%201%2E5%200%20002%2E112%202%2E13%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PaperClipIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M18.375 12.739l-7.693 7.693a4.5 4.5 0 01-6.364-6.364l10.94-10.94A3 3 0 1119.5 7.372L8.552 18.32m.009-.01l-.01.01m5.699-9.941l-7.81 7.81a1.5 1.5 0 002.112 2.13"/>
</svg>
  }
}
