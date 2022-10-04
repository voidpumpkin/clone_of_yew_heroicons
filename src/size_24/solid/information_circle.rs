use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%2012c0%2D5%2E385%204%2E365%2D9%2E75%209%2E75%2D9%2E75s9%2E75%204%2E365%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75%209%2E75S2%2E25%2017%2E385%202%2E25%2012zm8%2E706%2D1%2E442c1%2E146%2D%2E573%202%2E437%2E463%202%2E126%201%2E706l%2D%2E709%202%2E836%2E042%2D%2E02a%2E75%2E75%200%2001%2E67%201%2E34l%2D%2E04%2E022c%2D1%2E147%2E573%2D2%2E438%2D%2E463%2D2%2E127%2D1%2E706l%2E71%2D2%2E836%2D%2E042%2E02a%2E75%2E75%200%2011%2D%2E671%2D1%2E34l%2E041%2D%2E022zM12%209a%2E75%2E75%200%20100%2D1%2E5%2E75%2E75%200%20000%201%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn InformationCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm8.706-1.442c1.146-.573 2.437.463 2.126 1.706l-.709 2.836.042-.02a.75.75 0 01.67 1.34l-.04.022c-1.147.573-2.438-.463-2.127-1.706l.71-2.836-.042.02a.75.75 0 11-.671-1.34l.041-.022zM12 9a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd"/>
</svg>
  }
}