use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%203%2E5A1%2E5%201%2E5%200%20014%2E5%202h6%2E879a1%2E5%201%2E5%200%20011%2E06%2E44l4%2E122%204%2E12A1%2E5%201%2E5%200%200117%207%2E622V16%2E5a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5h%2D11A1%2E5%201%2E5%200%20013%2016%2E5v%2D13zm10%2E857%205%2E691a%2E75%2E75%200%2000%2D1%2E214%2D%2E882l%2D3%2E483%204%2E79%2D1%2E88%2D1%2E88a%2E75%2E75%200%2000%2D1%2E06%201%2E061l2%2E5%202%2E5a%2E75%2E75%200%20001%2E137%2D%2E089l4%2D5%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DocumentCheckIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 3.5A1.5 1.5 0 014.5 2h6.879a1.5 1.5 0 011.06.44l4.122 4.12A1.5 1.5 0 0117 7.622V16.5a1.5 1.5 0 01-1.5 1.5h-11A1.5 1.5 0 013 16.5v-13zm10.857 5.691a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 00-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"/>
</svg>
  }
}