use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E75%208a%2E75%2E75%200%2000%2D%2E75%2E75v2%2E5c0%20%2E414%2E336%2E75%2E75%2E75H9%2E5a%2E75%2E75%200%2000%2E75%2D%2E75v%2D2%2E5A%2E75%2E75%200%20009%2E5%208H4%2E75z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E25%205A2%2E25%202%2E25%200%20001%207%2E25v5%2E5A2%2E25%202%2E25%200%20003%2E25%2015h12%2E5A2%2E25%202%2E25%200%200018%2012%2E75v%2D1%2E085a1%2E5%201%2E5%200%20001%2D1%2E415v%2D%2E5a1%2E5%201%2E5%200%2000%2D1%2D1%2E415V7%2E25A2%2E25%202%2E25%200%200015%2E75%205H3%2E25zM2%2E5%207%2E25a%2E75%2E75%200%2001%2E75%2D%2E75h12%2E5a%2E75%2E75%200%2001%2E75%2E75v5%2E5a%2E75%2E75%200%2001%2D%2E75%2E75H3%2E25a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D5%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Battery50Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M4.75 8a.75.75 0 00-.75.75v2.5c0 .414.336.75.75.75H9.5a.75.75 0 00.75-.75v-2.5A.75.75 0 009.5 8H4.75z"/>
  <path fill-rule="evenodd" d="M3.25 5A2.25 2.25 0 001 7.25v5.5A2.25 2.25 0 003.25 15h12.5A2.25 2.25 0 0018 12.75v-1.085a1.5 1.5 0 001-1.415v-.5a1.5 1.5 0 00-1-1.415V7.25A2.25 2.25 0 0015.75 5H3.25zM2.5 7.25a.75.75 0 01.75-.75h12.5a.75.75 0 01.75.75v5.5a.75.75 0 01-.75.75H3.25a.75.75 0 01-.75-.75v-5.5z" clip-rule="evenodd"/>
</svg>
  }
}
