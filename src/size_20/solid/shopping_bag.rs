use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%205v1H4%2E667a1%2E75%201%2E75%200%2000%2D1%2E743%201%2E598l%2D%2E826%209%2E5A1%2E75%201%2E75%200%20003%2E84%2019H16%2E16a1%2E75%201%2E75%200%20001%2E743%2D1%2E902l%2D%2E826%2D9%2E5A1%2E75%201%2E75%200%200015%2E333%206H14V5a4%204%200%2000%2D8%200zm4%2D2%2E5A2%2E5%202%2E5%200%20007%2E5%205v1h5V5A2%2E5%202%2E5%200%200010%202%2E5zM7%2E5%2010a2%2E5%202%2E5%200%20005%200V8%2E75a%2E75%2E75%200%20011%2E5%200V10a4%204%200%2001%2D8%200V8%2E75a%2E75%2E75%200%20011%2E5%200V10z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShoppingBagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6 5v1H4.667a1.75 1.75 0 00-1.743 1.598l-.826 9.5A1.75 1.75 0 003.84 19H16.16a1.75 1.75 0 001.743-1.902l-.826-9.5A1.75 1.75 0 0015.333 6H14V5a4 4 0 00-8 0zm4-2.5A2.5 2.5 0 007.5 5v1h5V5A2.5 2.5 0 0010 2.5zM7.5 10a2.5 2.5 0 005 0V8.75a.75.75 0 011.5 0V10a4 4 0 01-8 0V8.75a.75.75 0 011.5 0V10z" clip-rule="evenodd"/>
</svg>
  }
}