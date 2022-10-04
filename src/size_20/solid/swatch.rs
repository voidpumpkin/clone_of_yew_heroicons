use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E5%202A1%2E5%201%2E5%200%20002%203%2E5V15a3%203%200%20106%200V3%2E5A1%2E5%201%2E5%200%20006%2E5%202h%2D3zm11%2E753%206%2E99L9%2E5%2014%2E743V6%2E257l1%2E51%2D1%2E51a1%2E5%201%2E5%200%20012%2E122%200l2%2E121%202%2E121a1%2E5%201%2E5%200%20010%202%2E122zM8%2E364%2018H16%2E5a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D3a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5h%2D2%2E136l%2D6%206zM5%2016a1%201%200%20100%2D2%201%201%200%20000%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SwatchIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.5 2A1.5 1.5 0 002 3.5V15a3 3 0 106 0V3.5A1.5 1.5 0 006.5 2h-3zm11.753 6.99L9.5 14.743V6.257l1.51-1.51a1.5 1.5 0 012.122 0l2.121 2.121a1.5 1.5 0 010 2.122zM8.364 18H16.5a1.5 1.5 0 001.5-1.5v-3a1.5 1.5 0 00-1.5-1.5h-2.136l-6 6zM5 16a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
</svg>
  }
}