use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2E362%201%2E093a%2E75%2E75%200%2000%2D%2E724%200L2%2E523%205%2E018%2010%209%2E143l7%2E477%2D4%2E125%2D7%2E115%2D3%2E925zM18%206%2E443l%2D7%2E25%204v8%2E25l6%2E862%2D3%2E786A%2E75%2E75%200%200018%2014%2E25V6%2E443zm%2D8%2E75%2012%2E25v%2D8%2E25l%2D7%2E25%2D4v7%2E807a%2E75%2E75%200%2000%2E388%2E657l6%2E862%203%2E786z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CubeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10.362 1.093a.75.75 0 00-.724 0L2.523 5.018 10 9.143l7.477-4.125-7.115-3.925zM18 6.443l-7.25 4v8.25l6.862-3.786A.75.75 0 0018 14.25V6.443zm-8.75 12.25v-8.25l-7.25-4v7.807a.75.75 0 00.388.657l6.862 3.786z" clip-rule="evenodd"/>
</svg>
  }
}