use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%2E75%209%2E776c%2E112%2D%2E017%2E227%2D%2E026%2E344%2D%2E026h15%2E812c%2E117%200%20%2E232%2E009%2E344%2E026m%2D16%2E5%200a2%2E25%202%2E25%200%2000%2D1%2E883%202%2E542l%2E857%206a2%2E25%202%2E25%200%20002%2E227%201%2E932H19%2E05a2%2E25%202%2E25%200%20002%2E227%2D1%2E932l%2E857%2D6a2%2E25%202%2E25%200%2000%2D1%2E883%2D2%2E542m%2D16%2E5%200V6A2%2E25%202%2E25%200%20016%203%2E75h3%2E879a1%2E5%201%2E5%200%20011%2E06%2E44l2%2E122%202%2E12a1%2E5%201%2E5%200%20001%2E06%2E44H18A2%2E25%202%2E25%200%200120%2E25%209v%2E776%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FolderOpenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 00-1.883 2.542l.857 6a2.25 2.25 0 002.227 1.932H19.05a2.25 2.25 0 002.227-1.932l.857-6a2.25 2.25 0 00-1.883-2.542m-16.5 0V6A2.25 2.25 0 016 3.75h3.879a1.5 1.5 0 011.06.44l2.122 2.12a1.5 1.5 0 001.06.44H18A2.25 2.25 0 0120.25 9v.776"/>
</svg>
  }
}