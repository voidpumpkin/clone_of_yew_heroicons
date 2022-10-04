use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%2E25%202A2%2E25%202%2E25%200%20002%204%2E25v11%2E5A2%2E25%202%2E25%200%20004%2E25%2018h11%2E5A2%2E25%202%2E25%200%200018%2015%2E75V4%2E25A2%2E25%202%2E25%200%200015%2E75%202H4%2E25zM6%2013%2E25V3%2E5h8v9%2E75a%2E75%2E75%200%2001%2D1%2E064%2E681L10%2012%2E576l%2D2%2E936%201%2E355A%2E75%2E75%200%20016%2013%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BookmarkSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M4.25 2A2.25 2.25 0 002 4.25v11.5A2.25 2.25 0 004.25 18h11.5A2.25 2.25 0 0018 15.75V4.25A2.25 2.25 0 0015.75 2H4.25zM6 13.25V3.5h8v9.75a.75.75 0 01-1.064.681L10 12.576l-2.936 1.355A.75.75 0 016 13.25z" clip-rule="evenodd"/>
</svg>
  }
}