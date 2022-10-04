use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M16%2E403%2012%2E652a3%203%200%20000%2D5%2E304%203%203%200%2000%2D3%2E75%2D3%2E751%203%203%200%2000%2D5%2E305%200%203%203%200%2000%2D3%2E751%203%2E75%203%203%200%20000%205%2E305%203%203%200%20003%2E75%203%2E751%203%203%200%20005%2E305%200%203%203%200%20003%2E751%2D3%2E75zm%2D2%2E546%2D4%2E46a%2E75%2E75%200%2000%2D1%2E214%2D%2E883l%2D3%2E483%204%2E79%2D1%2E88%2D1%2E88a%2E75%2E75%200%2010%2D1%2E06%201%2E061l2%2E5%202%2E5a%2E75%2E75%200%20001%2E137%2D%2E089l4%2D5%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CheckBadgeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M16.403 12.652a3 3 0 000-5.304 3 3 0 00-3.75-3.751 3 3 0 00-5.305 0 3 3 0 00-3.751 3.75 3 3 0 000 5.305 3 3 0 003.75 3.751 3 3 0 005.305 0 3 3 0 003.751-3.75zm-2.546-4.46a.75.75 0 00-1.214-.883l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"/>
</svg>
  }
}
