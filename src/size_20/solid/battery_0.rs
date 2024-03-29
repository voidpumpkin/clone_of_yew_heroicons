use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%207%2E25A2%2E25%202%2E25%200%20013%2E25%205h12%2E5A2%2E25%202%2E25%200%200118%207%2E25v1%2E085a1%2E5%201%2E5%200%20011%201%2E415v%2E5a1%2E5%201%2E5%200%2001%2D1%201%2E415v1%2E085A2%2E25%202%2E25%200%200115%2E75%2015H3%2E25A2%2E25%202%2E25%200%20011%2012%2E75v%2D5%2E5zm2%2E25%2D%2E75a%2E75%2E75%200%2000%2D%2E75%2E75v5%2E5c0%20%2E414%2E336%2E75%2E75%2E75h12%2E5a%2E75%2E75%200%2000%2E75%2D%2E75v%2D5%2E5a%2E75%2E75%200%2000%2D%2E75%2D%2E75H3%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Battery0Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1 7.25A2.25 2.25 0 013.25 5h12.5A2.25 2.25 0 0118 7.25v1.085a1.5 1.5 0 011 1.415v.5a1.5 1.5 0 01-1 1.415v1.085A2.25 2.25 0 0115.75 15H3.25A2.25 2.25 0 011 12.75v-5.5zm2.25-.75a.75.75 0 00-.75.75v5.5c0 .414.336.75.75.75h12.5a.75.75 0 00.75-.75v-5.5a.75.75 0 00-.75-.75H3.25z" clip-rule="evenodd"/>
</svg>
  }
}
