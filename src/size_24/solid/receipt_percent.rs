use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%201%2E5c%2D1%2E921%200%2D3%2E816%2E111%2D5%2E68%2E327%2D1%2E497%2E174%2D2%2E57%201%2E46%2D2%2E57%202%2E93V21%2E75a%2E75%2E75%200%20001%2E029%2E696l3%2E471%2D1%2E388%203%2E472%201%2E388a%2E75%2E75%200%2000%2E556%200l3%2E472%2D1%2E388%203%2E471%201%2E388a%2E75%2E75%200%20001%2E029%2D%2E696V4%2E757c0%2D1%2E47%2D1%2E073%2D2%2E756%2D2%2E57%2D2%2E93A49%2E255%2049%2E255%200%200012%201%2E5zm3%2E53%207%2E28a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06l%2D6%206a%2E75%2E75%200%20101%2E06%201%2E06l6%2D6zM8%2E625%209a1%2E125%201%2E125%200%20112%2E25%200%201%2E125%201%2E125%200%2001%2D2%2E25%200zm5%2E625%203%2E375a1%2E125%201%2E125%200%20100%202%2E25%201%2E125%201%2E125%200%20000%2D2%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ReceiptPercentIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 1.5c-1.921 0-3.816.111-5.68.327-1.497.174-2.57 1.46-2.57 2.93V21.75a.75.75 0 001.029.696l3.471-1.388 3.472 1.388a.75.75 0 00.556 0l3.472-1.388 3.471 1.388a.75.75 0 001.029-.696V4.757c0-1.47-1.073-2.756-2.57-2.93A49.255 49.255 0 0012 1.5zm3.53 7.28a.75.75 0 00-1.06-1.06l-6 6a.75.75 0 101.06 1.06l6-6zM8.625 9a1.125 1.125 0 112.25 0 1.125 1.125 0 01-2.25 0zm5.625 3.375a1.125 1.125 0 100 2.25 1.125 1.125 0 000-2.25z" clip-rule="evenodd"/>
</svg>
  }
}