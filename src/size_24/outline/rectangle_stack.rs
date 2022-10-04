use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M6%206%2E878V6a2%2E25%202%2E25%200%20012%2E25%2D2%2E25h7%2E5A2%2E25%202%2E25%200%200118%206v%2E878m%2D12%200c%2E235%2D%2E083%2E487%2D%2E128%2E75%2D%2E128h10%2E5c%2E263%200%20%2E515%2E045%2E75%2E128m%2D12%200A2%2E25%202%2E25%200%20004%2E5%209v%2E878m13%2E5%2D3A2%2E25%202%2E25%200%200119%2E5%209v%2E878m0%200a2%2E246%202%2E246%200%2000%2D%2E75%2D%2E128H5%2E25c%2D%2E263%200%2D%2E515%2E045%2D%2E75%2E128m15%200A2%2E25%202%2E25%200%200121%2012v6a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25H5%2E25A2%2E25%202%2E25%200%20013%2018v%2D6c0%2D%2E98%2E626%2D1%2E813%201%2E5%2D2%2E122%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RectangleStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M6 6.878V6a2.25 2.25 0 012.25-2.25h7.5A2.25 2.25 0 0118 6v.878m-12 0c.235-.083.487-.128.75-.128h10.5c.263 0 .515.045.75.128m-12 0A2.25 2.25 0 004.5 9v.878m13.5-3A2.25 2.25 0 0119.5 9v.878m0 0a2.246 2.246 0 00-.75-.128H5.25c-.263 0-.515.045-.75.128m15 0A2.25 2.25 0 0121 12v6a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 18v-6c0-.98.626-1.813 1.5-2.122"/>
</svg>
  }
}