use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%208c0%2D%2E26%2E017%2D%2E517%2E049%2D%2E77l7%2E722%207%2E723a33%2E56%2033%2E56%200%2001%2D3%2E722%2D%2E01%202%202%200%20003%2E862%2E15l1%2E134%201%2E134a3%2E5%203%2E5%200%2001%2D6%2E53%2D1%2E409%2032%2E91%2032%2E91%200%2001%2D3%2E257%2D%2E508%2E75%2E75%200%2001%2D%2E515%2D1%2E076A11%2E448%2011%2E448%200%20004%208zM17%2E266%2013%2E9a%2E756%2E756%200%2001%2D%2E068%2E116L6%2E389%203%2E207A6%206%200%200116%208c%2E001%201%2E887%2E455%203%2E665%201%2E258%205%2E234a%2E75%2E75%200%2001%2E01%2E666zM3%2E28%202%2E22a%2E75%2E75%200%2000%2D1%2E06%201%2E06l14%2E5%2014%2E5a%2E75%2E75%200%20101%2E06%2D1%2E06L3%2E28%202%2E22z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BellSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M4 8c0-.26.017-.517.049-.77l7.722 7.723a33.56 33.56 0 01-3.722-.01 2 2 0 003.862.15l1.134 1.134a3.5 3.5 0 01-6.53-1.409 32.91 32.91 0 01-3.257-.508.75.75 0 01-.515-1.076A11.448 11.448 0 004 8zM17.266 13.9a.756.756 0 01-.068.116L6.389 3.207A6 6 0 0116 8c.001 1.887.455 3.665 1.258 5.234a.75.75 0 01.01.666zM3.28 2.22a.75.75 0 00-1.06 1.06l14.5 14.5a.75.75 0 101.06-1.06L3.28 2.22z"/>
</svg>
  }
}