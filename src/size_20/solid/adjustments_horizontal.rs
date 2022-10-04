use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%203%2E75a2%202%200%2010%2D4%200%202%202%200%20004%200zM17%2E25%204%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D5%2E5a%2E75%2E75%200%20000%201%2E5h5%2E5zM5%203%2E75a%2E75%2E75%200%2001%2D%2E75%2E75h%2D1%2E5a%2E75%2E75%200%20010%2D1%2E5h1%2E5a%2E75%2E75%200%2001%2E75%2E75zM4%2E25%2017a%2E75%2E75%200%20000%2D1%2E5h%2D1%2E5a%2E75%2E75%200%20000%201%2E5h1%2E5zM17%2E25%2017a%2E75%2E75%200%20000%2D1%2E5h%2D5%2E5a%2E75%2E75%200%20000%201%2E5h5%2E5zM9%2010a%2E75%2E75%200%2001%2D%2E75%2E75h%2D5%2E5a%2E75%2E75%200%20010%2D1%2E5h5%2E5A%2E75%2E75%200%20019%2010zM17%2E25%2010%2E75a%2E75%2E75%200%20000%2D1%2E5h%2D1%2E5a%2E75%2E75%200%20000%201%2E5h1%2E5zM14%2010a2%202%200%2010%2D4%200%202%202%200%20004%200zM10%2016%2E25a2%202%200%2010%2D4%200%202%202%200%20004%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn AdjustmentsHorizontalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10 3.75a2 2 0 10-4 0 2 2 0 004 0zM17.25 4.5a.75.75 0 000-1.5h-5.5a.75.75 0 000 1.5h5.5zM5 3.75a.75.75 0 01-.75.75h-1.5a.75.75 0 010-1.5h1.5a.75.75 0 01.75.75zM4.25 17a.75.75 0 000-1.5h-1.5a.75.75 0 000 1.5h1.5zM17.25 17a.75.75 0 000-1.5h-5.5a.75.75 0 000 1.5h5.5zM9 10a.75.75 0 01-.75.75h-5.5a.75.75 0 010-1.5h5.5A.75.75 0 019 10zM17.25 10.75a.75.75 0 000-1.5h-1.5a.75.75 0 000 1.5h1.5zM14 10a2 2 0 10-4 0 2 2 0 004 0zM10 16.25a2 2 0 10-4 0 2 2 0 004 0z"/>
</svg>
  }
}
