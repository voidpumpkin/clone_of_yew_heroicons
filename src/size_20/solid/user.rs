use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%208a3%203%200%20100%2D6%203%203%200%20000%206zM3%2E465%2014%2E493a1%2E23%201%2E23%200%2000%2E41%201%2E412A9%2E957%209%2E957%200%200010%2018c2%2E31%200%204%2E438%2D%2E784%206%2E131%2D2%2E1%2E43%2D%2E333%2E604%2D%2E903%2E408%2D1%2E41a7%2E002%207%2E002%200%2000%2D13%2E074%2E003z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10 8a3 3 0 100-6 3 3 0 000 6zM3.465 14.493a1.23 1.23 0 00.41 1.412A9.957 9.957 0 0010 18c2.31 0 4.438-.784 6.131-2.1.43-.333.604-.903.408-1.41a7.002 7.002 0 00-13.074.003z"/>
</svg>
  }
}
