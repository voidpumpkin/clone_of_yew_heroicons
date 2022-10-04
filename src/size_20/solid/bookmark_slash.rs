use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M17%204%2E517v9%2E301L5%2E433%202%2E252a41%2E44%2041%2E44%200%20019%2E637%2E058C16%2E194%202%2E45%2017%203%2E414%2017%204%2E517zM3%2017%2E25V6%2E182l10%2E654%2010%2E654L10%2015%2E082l%2D5%2E925%202%2E844A%2E75%2E75%200%20013%2017%2E25zM3%2E28%202%2E22a%2E75%2E75%200%2000%2D1%2E06%201%2E06l14%2E5%2014%2E5a%2E75%2E75%200%20101%2E06%2D1%2E06L3%2E28%202%2E22z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BookmarkSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M17 4.517v9.301L5.433 2.252a41.44 41.44 0 019.637.058C16.194 2.45 17 3.414 17 4.517zM3 17.25V6.182l10.654 10.654L10 15.082l-5.925 2.844A.75.75 0 013 17.25zM3.28 2.22a.75.75 0 00-1.06 1.06l14.5 14.5a.75.75 0 101.06-1.06L3.28 2.22z"/>
</svg>
  }
}