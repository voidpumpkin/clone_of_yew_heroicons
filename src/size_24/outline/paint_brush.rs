use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2E53%2016%2E122a3%203%200%2000%2D5%2E78%201%2E128%202%2E25%202%2E25%200%2001%2D2%2E4%202%2E245%204%2E5%204%2E5%200%20008%2E4%2D2%2E245c0%2D%2E399%2D%2E078%2D%2E78%2D%2E22%2D1%2E128zm0%200a15%2E998%2015%2E998%200%20003%2E388%2D1%2E62m%2D5%2E043%2D%2E025a15%2E994%2015%2E994%200%20011%2E622%2D3%2E395m3%2E42%203%2E42a15%2E995%2015%2E995%200%20004%2E764%2D4%2E648l3%2E876%2D5%2E814a1%2E151%201%2E151%200%2000%2D1%2E597%2D1%2E597L14%2E146%206%2E32a15%2E996%2015%2E996%200%2000%2D4%2E649%204%2E763m3%2E42%203%2E42a6%2E776%206%2E776%200%2000%2D3%2E42%2D3%2E42%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PaintBrushIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9.53 16.122a3 3 0 00-5.78 1.128 2.25 2.25 0 01-2.4 2.245 4.5 4.5 0 008.4-2.245c0-.399-.078-.78-.22-1.128zm0 0a15.998 15.998 0 003.388-1.62m-5.043-.025a15.994 15.994 0 011.622-3.395m3.42 3.42a15.995 15.995 0 004.764-4.648l3.876-5.814a1.151 1.151 0 00-1.597-1.597L14.146 6.32a15.996 15.996 0 00-4.649 4.763m3.42 3.42a6.776 6.776 0 00-3.42-3.42"/>
</svg>
  }
}
