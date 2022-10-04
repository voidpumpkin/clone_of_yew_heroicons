use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%204a1%201%200%20011%2D1h16a1%201%200%20011%201v8a1%201%200%2001%2D1%201H2a1%201%200%2001%2D1%2D1V4zm12%204a3%203%200%2011%2D6%200%203%203%200%20016%200zM4%209a1%201%200%20100%2D2%201%201%200%20000%202zm13%2D1a1%201%200%2011%2D2%200%201%201%200%20012%200zM1%2E75%2014%2E5a%2E75%2E75%200%20000%201%2E5c4%2E417%200%208%2E693%2E603%2012%2E749%201%2E73%201%2E111%2E309%202%2E251%2D%2E512%202%2E251%2D1%2E696v%2D%2E784a%2E75%2E75%200%2000%2D1%2E5%200v%2E784a%2E272%2E272%200%2001%2D%2E35%2E25A49%2E043%2049%2E043%200%20001%2E75%2014%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BanknotesIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1 4a1 1 0 011-1h16a1 1 0 011 1v8a1 1 0 01-1 1H2a1 1 0 01-1-1V4zm12 4a3 3 0 11-6 0 3 3 0 016 0zM4 9a1 1 0 100-2 1 1 0 000 2zm13-1a1 1 0 11-2 0 1 1 0 012 0zM1.75 14.5a.75.75 0 000 1.5c4.417 0 8.693.603 12.749 1.73 1.111.309 2.251-.512 2.251-1.696v-.784a.75.75 0 00-1.5 0v.784a.272.272 0 01-.35.25A49.043 49.043 0 001.75 14.5z" clip-rule="evenodd"/>
</svg>
  }
}
