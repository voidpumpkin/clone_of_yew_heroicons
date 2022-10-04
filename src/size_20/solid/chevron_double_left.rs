use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M15%2E79%2014%2E77a%2E75%2E75%200%2001%2D1%2E06%2E02l%2D4%2E5%2D4%2E25a%2E75%2E75%200%20010%2D1%2E08l4%2E5%2D4%2E25a%2E75%2E75%200%20111%2E04%201%2E08L11%2E832%2010l3%2E938%203%2E71a%2E75%2E75%200%2001%2E02%201%2E06zm%2D6%200a%2E75%2E75%200%2001%2D1%2E06%2E02l%2D4%2E5%2D4%2E25a%2E75%2E75%200%20010%2D1%2E08l4%2E5%2D4%2E25a%2E75%2E75%200%20111%2E04%201%2E08L5%2E832%2010l3%2E938%203%2E71a%2E75%2E75%200%2001%2E02%201%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChevronDoubleLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M15.79 14.77a.75.75 0 01-1.06.02l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 111.04 1.08L11.832 10l3.938 3.71a.75.75 0 01.02 1.06zm-6 0a.75.75 0 01-1.06.02l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 111.04 1.08L5.832 10l3.938 3.71a.75.75 0 01.02 1.06z" clip-rule="evenodd"/>
</svg>
  }
}