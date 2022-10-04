use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E25%203v1%2E5M4%2E5%208%2E25H3m18%200h%2D1%2E5M4%2E5%2012H3m18%200h%2D1%2E5m%2D15%203%2E75H3m18%200h%2D1%2E5M8%2E25%2019%2E5V21M12%203v1%2E5m0%2015V21m3%2E75%2D18v1%2E5m0%2015V21m%2D9%2D1%2E5h10%2E5a2%2E25%202%2E25%200%20002%2E25%2D2%2E25V6%2E75a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H6%2E75A2%2E25%202%2E25%200%20004%2E5%206%2E75v10%2E5a2%2E25%202%2E25%200%20002%2E25%202%2E25zm%2E75%2D12h9v9h%2D9v%2D9z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CpuChipIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 3v1.5M4.5 8.25H3m18 0h-1.5M4.5 12H3m18 0h-1.5m-15 3.75H3m18 0h-1.5M8.25 19.5V21M12 3v1.5m0 15V21m3.75-18v1.5m0 15V21m-9-1.5h10.5a2.25 2.25 0 002.25-2.25V6.75a2.25 2.25 0 00-2.25-2.25H6.75A2.25 2.25 0 004.5 6.75v10.5a2.25 2.25 0 002.25 2.25zm.75-12h9v9h-9v-9z"/>
</svg>
  }
}
