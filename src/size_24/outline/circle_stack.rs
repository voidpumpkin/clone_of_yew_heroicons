use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M20%2E25%206%2E375c0%202%2E278%2D3%2E694%204%2E125%2D8%2E25%204%2E125S3%2E75%208%2E653%203%2E75%206%2E375m16%2E5%200c0%2D2%2E278%2D3%2E694%2D4%2E125%2D8%2E25%2D4%2E125S3%2E75%204%2E097%203%2E75%206%2E375m16%2E5%200v11%2E25c0%202%2E278%2D3%2E694%204%2E125%2D8%2E25%204%2E125s%2D8%2E25%2D1%2E847%2D8%2E25%2D4%2E125V6%2E375m16%2E5%200v3%2E75m%2D16%2E5%2D3%2E75v3%2E75m16%2E5%200v3%2E75C20%2E25%2016%2E153%2016%2E556%2018%2012%2018s%2D8%2E25%2D1%2E847%2D8%2E25%2D4%2E125v%2D3%2E75m16%2E5%200c0%202%2E278%2D3%2E694%204%2E125%2D8%2E25%204%2E125s%2D8%2E25%2D1%2E847%2D8%2E25%2D4%2E125%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CircleStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0v3.75C20.25 16.153 16.556 18 12 18s-8.25-1.847-8.25-4.125v-3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125"/>
</svg>
  }
}
