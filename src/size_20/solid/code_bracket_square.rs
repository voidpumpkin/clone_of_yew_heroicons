use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%2E25%202A2%2E25%202%2E25%200%20002%204%2E25v11%2E5A2%2E25%202%2E25%200%20004%2E25%2018h11%2E5A2%2E25%202%2E25%200%200018%2015%2E75V4%2E25A2%2E25%202%2E25%200%200015%2E75%202H4%2E25zm4%2E03%206%2E28a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06L4%2E97%209%2E47a%2E75%2E75%200%20000%201%2E06l2%2E25%202%2E25a%2E75%2E75%200%20001%2E06%2D1%2E06L6%2E56%2010l1%2E72%2D1%2E72zm4%2E5%2D1%2E06a%2E75%2E75%200%2010%2D1%2E06%201%2E06L13%2E44%2010l%2D1%2E72%201%2E72a%2E75%2E75%200%20101%2E06%201%2E06l2%2E25%2D2%2E25a%2E75%2E75%200%20000%2D1%2E06l%2D2%2E25%2D2%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CodeBracketSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M4.25 2A2.25 2.25 0 002 4.25v11.5A2.25 2.25 0 004.25 18h11.5A2.25 2.25 0 0018 15.75V4.25A2.25 2.25 0 0015.75 2H4.25zm4.03 6.28a.75.75 0 00-1.06-1.06L4.97 9.47a.75.75 0 000 1.06l2.25 2.25a.75.75 0 001.06-1.06L6.56 10l1.72-1.72zm4.5-1.06a.75.75 0 10-1.06 1.06L13.44 10l-1.72 1.72a.75.75 0 101.06 1.06l2.25-2.25a.75.75 0 000-1.06l-2.25-2.25z" clip-rule="evenodd"/>
</svg>
  }
}
