use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2E143%2017%2E082a24%2E248%2024%2E248%200%20003%2E844%2E148m%2D3%2E844%2D%2E148a23%2E856%2023%2E856%200%2001%2D5%2E455%2D1%2E31%208%2E964%208%2E964%200%20002%2E3%2D5%2E542m3%2E155%206%2E852a3%203%200%20005%2E667%201%2E97m1%2E965%2D2%2E277L21%2021m%2D4%2E225%2D4%2E225a23%2E81%2023%2E81%200%20003%2E536%2D1%2E003A8%2E967%208%2E967%200%200118%209%2E75V9A6%206%200%20006%2E53%206%2E53m10%2E245%2010%2E245L6%2E53%206%2E53M3%203l3%2E53%203%2E53%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BellSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9.143 17.082a24.248 24.248 0 003.844.148m-3.844-.148a23.856 23.856 0 01-5.455-1.31 8.964 8.964 0 002.3-5.542m3.155 6.852a3 3 0 005.667 1.97m1.965-2.277L21 21m-4.225-4.225a23.81 23.81 0 003.536-1.003A8.967 8.967 0 0118 9.75V9A6 6 0 006.53 6.53m10.245 10.245L6.53 6.53M3 3l3.53 3.53"/>
</svg>
  }
}
