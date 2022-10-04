use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%204%2E25A2%2E25%202%2E25%200%20015%2E25%202h5%2E5A2%2E25%202%2E25%200%200113%204%2E25v2a%2E75%2E75%200%2001%2D1%2E5%200v%2D2a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D5%2E5a%2E75%2E75%200%2000%2D%2E75%2E75v11%2E5c0%20%2E414%2E336%2E75%2E75%2E75h5%2E5a%2E75%2E75%200%2000%2E75%2D%2E75v%2D2a%2E75%2E75%200%20011%2E5%200v2A2%2E25%202%2E25%200%200110%2E75%2018h%2D5%2E5A2%2E25%202%2E25%200%20013%2015%2E75V4%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M19%2010a%2E75%2E75%200%2000%2D%2E75%2D%2E75H8%2E704l1%2E048%2D%2E943a%2E75%2E75%200%2010%2D1%2E004%2D1%2E114l%2D2%2E5%202%2E25a%2E75%2E75%200%20000%201%2E114l2%2E5%202%2E25a%2E75%2E75%200%20101%2E004%2D1%2E114l%2D1%2E048%2D%2E943h9%2E546A%2E75%2E75%200%200019%2010z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowLeftOnRectangleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 4.25A2.25 2.25 0 015.25 2h5.5A2.25 2.25 0 0113 4.25v2a.75.75 0 01-1.5 0v-2a.75.75 0 00-.75-.75h-5.5a.75.75 0 00-.75.75v11.5c0 .414.336.75.75.75h5.5a.75.75 0 00.75-.75v-2a.75.75 0 011.5 0v2A2.25 2.25 0 0110.75 18h-5.5A2.25 2.25 0 013 15.75V4.25z" clip-rule="evenodd"/>
  <path fill-rule="evenodd" d="M19 10a.75.75 0 00-.75-.75H8.704l1.048-.943a.75.75 0 10-1.004-1.114l-2.5 2.25a.75.75 0 000 1.114l2.5 2.25a.75.75 0 101.004-1.114l-1.048-.943h9.546A.75.75 0 0019 10z" clip-rule="evenodd"/>
</svg>
  }
}
