use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M12%202%2E25a%2E75%2E75%200%2001%2E75%2E75v2%2E25a%2E75%2E75%200%2001%2D1%2E5%200V3a%2E75%2E75%200%2001%2E75%2D%2E75zM7%2E5%2012a4%2E5%204%2E5%200%20119%200%204%2E5%204%2E5%200%2001%2D9%200zM18%2E894%206%2E166a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06l%2D1%2E591%201%2E59a%2E75%2E75%200%20101%2E06%201%2E061l1%2E591%2D1%2E59zM21%2E75%2012a%2E75%2E75%200%2001%2D%2E75%2E75h%2D2%2E25a%2E75%2E75%200%20010%2D1%2E5H21a%2E75%2E75%200%2001%2E75%2E75zM17%2E834%2018%2E894a%2E75%2E75%200%20001%2E06%2D1%2E06l%2D1%2E59%2D1%2E591a%2E75%2E75%200%2010%2D1%2E061%201%2E06l1%2E59%201%2E591zM12%2018a%2E75%2E75%200%2001%2E75%2E75V21a%2E75%2E75%200%2001%2D1%2E5%200v%2D2%2E25A%2E75%2E75%200%200112%2018zM7%2E758%2017%2E303a%2E75%2E75%200%2000%2D1%2E061%2D1%2E06l%2D1%2E591%201%2E59a%2E75%2E75%200%20001%2E06%201%2E061l1%2E591%2D1%2E59zM6%2012a%2E75%2E75%200%2001%2D%2E75%2E75H3a%2E75%2E75%200%20010%2D1%2E5h2%2E25A%2E75%2E75%200%20016%2012zM6%2E697%207%2E757a%2E75%2E75%200%20001%2E06%2D1%2E06l%2D1%2E59%2D1%2E591a%2E75%2E75%200%2000%2D1%2E061%201%2E06l1%2E59%201%2E591z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SunIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M12 2.25a.75.75 0 01.75.75v2.25a.75.75 0 01-1.5 0V3a.75.75 0 01.75-.75zM7.5 12a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM18.894 6.166a.75.75 0 00-1.06-1.06l-1.591 1.59a.75.75 0 101.06 1.061l1.591-1.59zM21.75 12a.75.75 0 01-.75.75h-2.25a.75.75 0 010-1.5H21a.75.75 0 01.75.75zM17.834 18.894a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 10-1.061 1.06l1.59 1.591zM12 18a.75.75 0 01.75.75V21a.75.75 0 01-1.5 0v-2.25A.75.75 0 0112 18zM7.758 17.303a.75.75 0 00-1.061-1.06l-1.591 1.59a.75.75 0 001.06 1.061l1.591-1.59zM6 12a.75.75 0 01-.75.75H3a.75.75 0 010-1.5h2.25A.75.75 0 016 12zM6.697 7.757a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 00-1.061 1.06l1.59 1.591z"/>
</svg>
  }
}
