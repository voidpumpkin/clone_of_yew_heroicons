use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%204%2E75C2%203%2E784%202%2E784%203%203%2E75%203h4%2E836c%2E464%200%20%2E909%2E184%201%2E237%2E513l1%2E414%201%2E414a%2E25%2E25%200%2000%2E177%2E073h4%2E836c%2E966%200%201%2E75%2E784%201%2E75%201%2E75v8%2E5A1%2E75%201%2E75%200%200116%2E25%2017H3%2E75A1%2E75%201%2E75%200%20012%2015%2E25V4%2E75zm8%2E75%204a%2E75%2E75%200%2000%2D1%2E5%200v2%2E546l%2D%2E943%2D1%2E048a%2E75%2E75%200%2010%2D1%2E114%201%2E004l2%2E25%202%2E5a%2E75%2E75%200%20001%2E114%200l2%2E25%2D2%2E5a%2E75%2E75%200%2010%2D1%2E114%2D1%2E004l%2D%2E943%201%2E048V8%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FolderArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2 4.75C2 3.784 2.784 3 3.75 3h4.836c.464 0 .909.184 1.237.513l1.414 1.414a.25.25 0 00.177.073h4.836c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0116.25 17H3.75A1.75 1.75 0 012 15.25V4.75zm8.75 4a.75.75 0 00-1.5 0v2.546l-.943-1.048a.75.75 0 10-1.114 1.004l2.25 2.5a.75.75 0 001.114 0l2.25-2.5a.75.75 0 10-1.114-1.004l-.943 1.048V8.75z" clip-rule="evenodd"/>
</svg>
  }
}