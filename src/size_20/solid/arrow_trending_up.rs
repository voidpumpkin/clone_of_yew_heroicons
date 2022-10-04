use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%2E577%204%2E878a%2E75%2E75%200%2001%2E919%2D%2E53l4%2E78%201%2E281a%2E75%2E75%200%2001%2E531%2E919l%2D1%2E281%204%2E78a%2E75%2E75%200%2001%2D1%2E449%2D%2E387l%2E81%2D3%2E022a19%2E407%2019%2E407%200%2000%2D5%2E594%205%2E203%2E75%2E75%200%2001%2D1%2E139%2E093L7%2010%2E06l%2D4%2E72%204%2E72a%2E75%2E75%200%2001%2D1%2E06%2D1%2E061l5%2E25%2D5%2E25a%2E75%2E75%200%20011%2E06%200l3%2E074%203%2E073a20%2E923%2020%2E923%200%20015%2E545%2D4%2E931l%2D3%2E042%2D%2E815a%2E75%2E75%200%2001%2D%2E53%2D%2E919z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowTrendingUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12.577 4.878a.75.75 0 01.919-.53l4.78 1.281a.75.75 0 01.531.919l-1.281 4.78a.75.75 0 01-1.449-.387l.81-3.022a19.407 19.407 0 00-5.594 5.203.75.75 0 01-1.139.093L7 10.06l-4.72 4.72a.75.75 0 01-1.06-1.061l5.25-5.25a.75.75 0 011.06 0l3.074 3.073a20.923 20.923 0 015.545-4.931l-3.042-.815a.75.75 0 01-.53-.919z" clip-rule="evenodd"/>
</svg>
  }
}
