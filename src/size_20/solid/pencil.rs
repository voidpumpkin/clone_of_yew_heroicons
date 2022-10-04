use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M2%2E695%2014%2E763l%2D1%2E262%203%2E154a%2E5%2E5%200%2000%2E65%2E65l3%2E155%2D1%2E262a4%204%200%20001%2E343%2D%2E885L17%2E5%205%2E5a2%2E121%202%2E121%200%2000%2D3%2D3L3%2E58%2013%2E42a4%204%200%2000%2D%2E885%201%2E343z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PencilIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M2.695 14.763l-1.262 3.154a.5.5 0 00.65.65l3.155-1.262a4 4 0 001.343-.885L17.5 5.5a2.121 2.121 0 00-3-3L3.58 13.42a4 4 0 00-.885 1.343z"/>
</svg>
  }
}
