use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%2018v%2D5%2E25m0%200a6%2E01%206%2E01%200%20001%2E5%2D%2E189m%2D1%2E5%2E189a6%2E01%206%2E01%200%2001%2D1%2E5%2D%2E189m3%2E75%207%2E478a12%2E06%2012%2E06%200%2001%2D4%2E5%200m3%2E75%202%2E383a14%2E406%2014%2E406%200%2001%2D3%200M14%2E25%2018v%2D%2E192c0%2D%2E983%2E658%2D1%2E823%201%2E508%2D2%2E316a7%2E5%207%2E5%200%2010%2D7%2E517%200c%2E85%2E493%201%2E509%201%2E333%201%2E509%202%2E316V18%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LightBulbIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 18v-5.25m0 0a6.01 6.01 0 001.5-.189m-1.5.189a6.01 6.01 0 01-1.5-.189m3.75 7.478a12.06 12.06 0 01-4.5 0m3.75 2.383a14.406 14.406 0 01-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 10-7.517 0c.85.493 1.509 1.333 1.509 2.316V18"/>
</svg>
  }
}