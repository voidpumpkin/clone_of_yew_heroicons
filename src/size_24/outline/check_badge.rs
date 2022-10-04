use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2012%2E75L11%2E25%2015%2015%209%2E75M21%2012c0%201%2E268%2D%2E63%202%2E39%2D1%2E593%203%2E068a3%2E745%203%2E745%200%2001%2D1%2E043%203%2E296%203%2E745%203%2E745%200%2001%2D3%2E296%201%2E043A3%2E745%203%2E745%200%200112%2021c%2D1%2E268%200%2D2%2E39%2D%2E63%2D3%2E068%2D1%2E593a3%2E746%203%2E746%200%2001%2D3%2E296%2D1%2E043%203%2E745%203%2E745%200%2001%2D1%2E043%2D3%2E296A3%2E745%203%2E745%200%20013%2012c0%2D1%2E268%2E63%2D2%2E39%201%2E593%2D3%2E068a3%2E745%203%2E745%200%20011%2E043%2D3%2E296%203%2E746%203%2E746%200%20013%2E296%2D1%2E043A3%2E746%203%2E746%200%200112%203c1%2E268%200%202%2E39%2E63%203%2E068%201%2E593a3%2E746%203%2E746%200%20013%2E296%201%2E043%203%2E746%203%2E746%200%20011%2E043%203%2E296A3%2E745%203%2E745%200%200121%2012z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CheckBadgeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12c0 1.268-.63 2.39-1.593 3.068a3.745 3.745 0 01-1.043 3.296 3.745 3.745 0 01-3.296 1.043A3.745 3.745 0 0112 21c-1.268 0-2.39-.63-3.068-1.593a3.746 3.746 0 01-3.296-1.043 3.745 3.745 0 01-1.043-3.296A3.745 3.745 0 013 12c0-1.268.63-2.39 1.593-3.068a3.745 3.745 0 011.043-3.296 3.746 3.746 0 013.296-1.043A3.746 3.746 0 0112 3c1.268 0 2.39.63 3.068 1.593a3.746 3.746 0 013.296 1.043 3.746 3.746 0 011.043 3.296A3.745 3.745 0 0121 12z"/>
</svg>
  }
}
