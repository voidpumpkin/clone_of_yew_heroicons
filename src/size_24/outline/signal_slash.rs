use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%203l8%2E735%208%2E735m0%200a%2E374%2E374%200%2011%2E53%2E53m%2D%2E53%2D%2E53l%2E53%2E53m0%200L21%2021M14%2E652%209%2E348a3%2E75%203%2E75%200%20010%205%2E304m2%2E121%2D7%2E425a6%2E75%206%2E75%200%20010%209%2E546m2%2E121%2D11%2E667c3%2E808%203%2E807%203%2E808%209%2E98%200%2013%2E788m%2D9%2E546%2D4%2E242a3%2E733%203%2E733%200%2001%2D1%2E06%2D2%2E122m%2D1%2E061%204%2E243a6%2E75%206%2E75%200%2001%2D1%2E625%2D6%2E929m%2D%2E496%209%2E05c%2D3%2E068%2D3%2E067%2D3%2E664%2D7%2E67%2D1%2E79%2D11%2E334M12%2012h%2E008v%2E008H12V12z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SignalSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3 3l8.735 8.735m0 0a.374.374 0 11.53.53m-.53-.53l.53.53m0 0L21 21M14.652 9.348a3.75 3.75 0 010 5.304m2.121-7.425a6.75 6.75 0 010 9.546m2.121-11.667c3.808 3.807 3.808 9.98 0 13.788m-9.546-4.242a3.733 3.733 0 01-1.06-2.122m-1.061 4.243a6.75 6.75 0 01-1.625-6.929m-.496 9.05c-3.068-3.067-3.664-7.67-1.79-11.334M12 12h.008v.008H12V12z"/>
</svg>
  }
}