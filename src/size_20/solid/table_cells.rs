use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M%2E99%205%2E24A2%2E25%202%2E25%200%20013%2E25%203h13%2E5A2%2E25%202%2E25%200%200119%205%2E25l%2E01%209%2E5A2%2E25%202%2E25%200%200116%2E76%2017H3%2E26A2%2E267%202%2E267%200%20011%2014%2E74l%2D%2E01%2D9%2E5zm8%2E26%209%2E52v%2D%2E625a%2E75%2E75%200%2000%2D%2E75%2D%2E75H3%2E25a%2E75%2E75%200%2000%2D%2E75%2E75v%2E615c0%20%2E414%2E336%2E75%2E75%2E75h5%2E373a%2E75%2E75%200%2000%2E627%2D%2E74zm1%2E5%200a%2E75%2E75%200%2000%2E627%2E74h5%2E373a%2E75%2E75%200%2000%2E75%2D%2E75v%2D%2E615a%2E75%2E75%200%2000%2D%2E75%2D%2E75H11%2E5a%2E75%2E75%200%2000%2D%2E75%2E75v%2E625zm6%2E75%2D3%2E63v%2D%2E625a%2E75%2E75%200%2000%2D%2E75%2D%2E75H11%2E5a%2E75%2E75%200%2000%2D%2E75%2E75v%2E625c0%20%2E414%2E336%2E75%2E75%2E75h5%2E25a%2E75%2E75%200%2000%2E75%2D%2E75zm%2D8%2E25%200v%2D%2E625a%2E75%2E75%200%2000%2D%2E75%2D%2E75H3%2E25a%2E75%2E75%200%2000%2D%2E75%2E75v%2E625c0%20%2E414%2E336%2E75%2E75%2E75H8%2E5a%2E75%2E75%200%2000%2E75%2D%2E75zM17%2E5%207%2E5v%2D%2E625a%2E75%2E75%200%2000%2D%2E75%2D%2E75H11%2E5a%2E75%2E75%200%2000%2D%2E75%2E75V7%2E5c0%20%2E414%2E336%2E75%2E75%2E75h5%2E25a%2E75%2E75%200%2000%2E75%2D%2E75zm%2D8%2E25%200v%2D%2E625a%2E75%2E75%200%2000%2D%2E75%2D%2E75H3%2E25a%2E75%2E75%200%2000%2D%2E75%2E75V7%2E5c0%20%2E414%2E336%2E75%2E75%2E75H8%2E5a%2E75%2E75%200%2000%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TableCellsIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M.99 5.24A2.25 2.25 0 013.25 3h13.5A2.25 2.25 0 0119 5.25l.01 9.5A2.25 2.25 0 0116.76 17H3.26A2.267 2.267 0 011 14.74l-.01-9.5zm8.26 9.52v-.625a.75.75 0 00-.75-.75H3.25a.75.75 0 00-.75.75v.615c0 .414.336.75.75.75h5.373a.75.75 0 00.627-.74zm1.5 0a.75.75 0 00.627.74h5.373a.75.75 0 00.75-.75v-.615a.75.75 0 00-.75-.75H11.5a.75.75 0 00-.75.75v.625zm6.75-3.63v-.625a.75.75 0 00-.75-.75H11.5a.75.75 0 00-.75.75v.625c0 .414.336.75.75.75h5.25a.75.75 0 00.75-.75zm-8.25 0v-.625a.75.75 0 00-.75-.75H3.25a.75.75 0 00-.75.75v.625c0 .414.336.75.75.75H8.5a.75.75 0 00.75-.75zM17.5 7.5v-.625a.75.75 0 00-.75-.75H11.5a.75.75 0 00-.75.75V7.5c0 .414.336.75.75.75h5.25a.75.75 0 00.75-.75zm-8.25 0v-.625a.75.75 0 00-.75-.75H3.25a.75.75 0 00-.75.75V7.5c0 .414.336.75.75.75H8.5a.75.75 0 00.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}