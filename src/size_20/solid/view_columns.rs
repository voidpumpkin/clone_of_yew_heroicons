use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M14%2017h2%2E75A2%2E25%202%2E25%200%200019%2014%2E75v%2D9%2E5A2%2E25%202%2E25%200%200016%2E75%203H14v14zM12%2E5%203h%2D5v14h5V3zM3%2E25%203H6v14H3%2E25A2%2E25%202%2E25%200%20011%2014%2E75v%2D9%2E5A2%2E25%202%2E25%200%20013%2E25%203z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ViewColumnsIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M14 17h2.75A2.25 2.25 0 0019 14.75v-9.5A2.25 2.25 0 0016.75 3H14v14zM12.5 3h-5v14h5V3zM3.25 3H6v14H3.25A2.25 2.25 0 011 14.75v-9.5A2.25 2.25 0 013.25 3z"/>
</svg>
  }
}