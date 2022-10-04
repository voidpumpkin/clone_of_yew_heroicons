use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M20%2E25%203%2E75v4%2E5m0%2D4%2E5h%2D4%2E5m4%2E5%200l%2D6%206m3%2012c%2D8%2E284%200%2D15%2D6%2E716%2D15%2D15V4%2E5A2%2E25%202%2E25%200%20014%2E5%202%2E25h1%2E372c%2E516%200%20%2E966%2E351%201%2E091%2E852l1%2E106%204%2E423c%2E11%2E44%2D%2E054%2E902%2D%2E417%201%2E173l%2D1%2E293%2E97a1%2E062%201%2E062%200%2000%2D%2E38%201%2E21%2012%2E035%2012%2E035%200%20007%2E143%207%2E143c%2E441%2E162%2E928%2D%2E004%201%2E21%2D%2E38l%2E97%2D1%2E293a1%2E125%201%2E125%200%20011%2E173%2D%2E417l4%2E423%201%2E106c%2E5%2E125%2E852%2E575%2E852%201%2E091V19%2E5a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25h%2D2%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PhoneArrowUpRightIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 3.75v4.5m0-4.5h-4.5m4.5 0l-6 6m3 12c-8.284 0-15-6.716-15-15V4.5A2.25 2.25 0 014.5 2.25h1.372c.516 0 .966.351 1.091.852l1.106 4.423c.11.44-.054.902-.417 1.173l-1.293.97a1.062 1.062 0 00-.38 1.21 12.035 12.035 0 007.143 7.143c.441.162.928-.004 1.21-.38l.97-1.293a1.125 1.125 0 011.173-.417l4.423 1.106c.5.125.852.575.852 1.091V19.5a2.25 2.25 0 01-2.25 2.25h-2.25z"/>
</svg>
  }
}
