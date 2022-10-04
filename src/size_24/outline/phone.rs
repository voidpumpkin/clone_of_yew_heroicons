use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%206%2E75c0%208%2E284%206%2E716%2015%2015%2015h2%2E25a2%2E25%202%2E25%200%20002%2E25%2D2%2E25v%2D1%2E372c0%2D%2E516%2D%2E351%2D%2E966%2D%2E852%2D1%2E091l%2D4%2E423%2D1%2E106c%2D%2E44%2D%2E11%2D%2E902%2E055%2D1%2E173%2E417l%2D%2E97%201%2E293c%2D%2E282%2E376%2D%2E769%2E542%2D1%2E21%2E38a12%2E035%2012%2E035%200%2001%2D7%2E143%2D7%2E143c%2D%2E162%2D%2E441%2E004%2D%2E928%2E38%2D1%2E21l1%2E293%2D%2E97c%2E363%2D%2E271%2E527%2D%2E734%2E417%2D1%2E173L6%2E963%203%2E102a1%2E125%201%2E125%200%2000%2D1%2E091%2D%2E852H4%2E5A2%2E25%202%2E25%200%20002%2E25%204%2E5v2%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PhoneIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 6.75c0 8.284 6.716 15 15 15h2.25a2.25 2.25 0 002.25-2.25v-1.372c0-.516-.351-.966-.852-1.091l-4.423-1.106c-.44-.11-.902.055-1.173.417l-.97 1.293c-.282.376-.769.542-1.21.38a12.035 12.035 0 01-7.143-7.143c-.162-.441.004-.928.38-1.21l1.293-.97c.363-.271.527-.734.417-1.173L6.963 3.102a1.125 1.125 0 00-1.091-.852H4.5A2.25 2.25 0 002.25 4.5v2.25z"/>
</svg>
  }
}
