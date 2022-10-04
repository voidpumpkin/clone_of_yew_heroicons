use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M1%2013%2E75V7%2E182L9%2E818%2016H3%2E25A2%2E25%202%2E25%200%20011%2013%2E75zM13%206%2E25v6%2E568L4%2E182%204h6%2E568A2%2E25%202%2E25%200%200113%206%2E25zM19%204%2E75a%2E75%2E75%200%2000%2D1%2E28%2D%2E53l%2D3%203a%2E75%2E75%200%2000%2D%2E22%2E53v4%2E5c0%20%2E199%2E079%2E39%2E22%2E53l3%203a%2E75%2E75%200%20001%2E28%2D%2E53V4%2E75zM2%2E28%204%2E22a%2E75%2E75%200%2000%2D1%2E06%201%2E06l10%2E5%2010%2E5a%2E75%2E75%200%20101%2E06%2D1%2E06L2%2E28%204%2E22z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn VideoCameraSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M1 13.75V7.182L9.818 16H3.25A2.25 2.25 0 011 13.75zM13 6.25v6.568L4.182 4h6.568A2.25 2.25 0 0113 6.25zM19 4.75a.75.75 0 00-1.28-.53l-3 3a.75.75 0 00-.22.53v4.5c0 .199.079.39.22.53l3 3a.75.75 0 001.28-.53V4.75zM2.28 4.22a.75.75 0 00-1.06 1.06l10.5 10.5a.75.75 0 101.06-1.06L2.28 4.22z"/>
</svg>
  }
}
