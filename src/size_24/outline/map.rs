use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%206%2E75V15m6%2D6v8%2E25m%2E503%203%2E498l4%2E875%2D2%2E437c%2E381%2D%2E19%2E622%2D%2E58%2E622%2D1%2E006V4%2E82c0%2D%2E836%2D%2E88%2D1%2E38%2D1%2E628%2D1%2E006l%2D3%2E869%201%2E934c%2D%2E317%2E159%2D%2E69%2E159%2D1%2E006%200L9%2E503%203%2E252a1%2E125%201%2E125%200%2000%2D1%2E006%200L3%2E622%205%2E689C3%2E24%205%2E88%203%206%2E27%203%206%2E695V19%2E18c0%20%2E836%2E88%201%2E38%201%2E628%201%2E006l3%2E869%2D1%2E934c%2E317%2D%2E159%2E69%2D%2E159%201%2E006%200l4%2E994%202%2E497c%2E317%2E158%2E69%2E158%201%2E006%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MapIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 6.75V15m6-6v8.25m.503 3.498l4.875-2.437c.381-.19.622-.58.622-1.006V4.82c0-.836-.88-1.38-1.628-1.006l-3.869 1.934c-.317.159-.69.159-1.006 0L9.503 3.252a1.125 1.125 0 00-1.006 0L3.622 5.689C3.24 5.88 3 6.27 3 6.695V19.18c0 .836.88 1.38 1.628 1.006l3.869-1.934c.317-.159.69-.159 1.006 0l4.994 2.497c.317.158.69.158 1.006 0z"/>
</svg>
  }
}
