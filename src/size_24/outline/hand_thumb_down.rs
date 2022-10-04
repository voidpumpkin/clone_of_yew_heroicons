use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M7%2E5%2015h2%2E25m8%2E024%2D9%2E75c%2E011%2E05%2E028%2E1%2E052%2E148%2E591%201%2E2%2E924%202%2E55%2E924%203%2E977a8%2E96%208%2E96%200%2001%2D%2E999%204%2E125m%2E023%2D8%2E25c%2D%2E076%2D%2E365%2E183%2D%2E75%2E575%2D%2E75h%2E908c%2E889%200%201%2E713%2E518%201%2E972%201%2E368%2E339%201%2E11%2E521%202%2E287%2E521%203%2E507%200%201%2E553%2D%2E295%203%2E036%2D%2E831%204%2E398C20%2E613%2014%2E547%2019%2E833%2015%2019%2015h%2D1%2E053c%2D%2E472%200%2D%2E745%2D%2E556%2D%2E5%2D%2E96a8%2E95%208%2E95%200%2000%2E303%2D%2E54m%2E023%2D8%2E25H16%2E48a4%2E5%204%2E5%200%2001%2D1%2E423%2D%2E23l%2D3%2E114%2D1%2E04a4%2E5%204%2E5%200%2000%2D1%2E423%2D%2E23H6%2E504c%2D%2E618%200%2D1%2E217%2E247%2D1%2E605%2E729A11%2E95%2011%2E95%200%20002%2E25%2012c0%20%2E434%2E023%2E863%2E068%201%2E285C2%2E427%2014%2E306%203%2E346%2015%204%2E372%2015h3%2E126c%2E618%200%20%2E991%2E724%2E725%201%2E282A7%2E471%207%2E471%200%20007%2E5%2019%2E5a2%2E25%202%2E25%200%20002%2E25%202%2E25%2E75%2E75%200%2000%2E75%2D%2E75v%2D%2E633c0%2D%2E573%2E11%2D1%2E14%2E322%2D1%2E672%2E304%2D%2E76%2E93%2D1%2E33%201%2E653%2D1%2E715a9%2E04%209%2E04%200%20002%2E86%2D2%2E4c%2E498%2D%2E634%201%2E226%2D1%2E08%202%2E032%2D1%2E08h%2E384%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HandThumbDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 15h2.25m8.024-9.75c.011.05.028.1.052.148.591 1.2.924 2.55.924 3.977a8.96 8.96 0 01-.999 4.125m.023-8.25c-.076-.365.183-.75.575-.75h.908c.889 0 1.713.518 1.972 1.368.339 1.11.521 2.287.521 3.507 0 1.553-.295 3.036-.831 4.398C20.613 14.547 19.833 15 19 15h-1.053c-.472 0-.745-.556-.5-.96a8.95 8.95 0 00.303-.54m.023-8.25H16.48a4.5 4.5 0 01-1.423-.23l-3.114-1.04a4.5 4.5 0 00-1.423-.23H6.504c-.618 0-1.217.247-1.605.729A11.95 11.95 0 002.25 12c0 .434.023.863.068 1.285C2.427 14.306 3.346 15 4.372 15h3.126c.618 0 .991.724.725 1.282A7.471 7.471 0 007.5 19.5a2.25 2.25 0 002.25 2.25.75.75 0 00.75-.75v-.633c0-.573.11-1.14.322-1.672.304-.76.93-1.33 1.653-1.715a9.04 9.04 0 002.86-2.4c.498-.634 1.226-1.08 2.032-1.08h.384"/>
</svg>
  }
}
