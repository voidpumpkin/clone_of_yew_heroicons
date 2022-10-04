use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%206a3%203%200%20013%2D3h12a3%203%200%20013%203v12a3%203%200%2001%2D3%203H6a3%203%200%2001%2D3%2D3V6zm14%2E25%206a%2E75%2E75%200%2001%2D%2E22%2E53l%2D2%2E25%202%2E25a%2E75%2E75%200%2011%2D1%2E06%2D1%2E06L15%2E44%2012l%2D1%2E72%2D1%2E72a%2E75%2E75%200%20111%2E06%2D1%2E06l2%2E25%202%2E25c%2E141%2E14%2E22%2E331%2E22%2E53zm%2D10%2E28%2D%2E53a%2E75%2E75%200%20000%201%2E06l2%2E25%202%2E25a%2E75%2E75%200%20101%2E06%2D1%2E06L8%2E56%2012l1%2E72%2D1%2E72a%2E75%2E75%200%2010%2D1%2E06%2D1%2E06l%2D2%2E25%202%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CodeBracketSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 6a3 3 0 013-3h12a3 3 0 013 3v12a3 3 0 01-3 3H6a3 3 0 01-3-3V6zm14.25 6a.75.75 0 01-.22.53l-2.25 2.25a.75.75 0 11-1.06-1.06L15.44 12l-1.72-1.72a.75.75 0 111.06-1.06l2.25 2.25c.141.14.22.331.22.53zm-10.28-.53a.75.75 0 000 1.06l2.25 2.25a.75.75 0 101.06-1.06L8.56 12l1.72-1.72a.75.75 0 10-1.06-1.06l-2.25 2.25z" clip-rule="evenodd"/>
</svg>
  }
}
