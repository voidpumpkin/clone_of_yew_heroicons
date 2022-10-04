use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E105%202%2E289a%2E75%2E75%200%2000%2D%2E826%2E95l1%2E414%204%2E925A1%2E5%201%2E5%200%20005%2E135%209%2E25h6%2E115a%2E75%2E75%200%20010%201%2E5H5%2E135a1%2E5%201%2E5%200%2000%2D1%2E442%201%2E086l%2D1%2E414%204%2E926a%2E75%2E75%200%2000%2E826%2E95%2028%2E896%2028%2E896%200%200015%2E293%2D7%2E154%2E75%2E75%200%20000%2D1%2E115A28%2E897%2028%2E897%200%20003%2E105%202%2E289z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PaperAirplaneIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M3.105 2.289a.75.75 0 00-.826.95l1.414 4.925A1.5 1.5 0 005.135 9.25h6.115a.75.75 0 010 1.5H5.135a1.5 1.5 0 00-1.442 1.086l-1.414 4.926a.75.75 0 00.826.95 28.896 28.896 0 0015.293-7.154.75.75 0 000-1.115A28.897 28.897 0 003.105 2.289z"/>
</svg>
  }
}
