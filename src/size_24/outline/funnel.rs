use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%203c2%2E755%200%205%2E455%2E232%208%2E083%2E678%2E533%2E09%2E917%2E556%2E917%201%2E096v1%2E044a2%2E25%202%2E25%200%2001%2D%2E659%201%2E591l%2D5%2E432%205%2E432a2%2E25%202%2E25%200%2000%2D%2E659%201%2E591v2%2E927a2%2E25%202%2E25%200%2001%2D1%2E244%202%2E013L9%2E75%2021v%2D6%2E568a2%2E25%202%2E25%200%2000%2D%2E659%2D1%2E591L3%2E659%207%2E409A2%2E25%202%2E25%200%20013%205%2E818V4%2E774c0%2D%2E54%2E384%2D1%2E006%2E917%2D1%2E096A48%2E32%2048%2E32%200%200112%203z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FunnelIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 3c2.755 0 5.455.232 8.083.678.533.09.917.556.917 1.096v1.044a2.25 2.25 0 01-.659 1.591l-5.432 5.432a2.25 2.25 0 00-.659 1.591v2.927a2.25 2.25 0 01-1.244 2.013L9.75 21v-6.568a2.25 2.25 0 00-.659-1.591L3.659 7.409A2.25 2.25 0 013 5.818V4.774c0-.54.384-1.006.917-1.096A48.32 48.32 0 0112 3z"/>
</svg>
  }
}
