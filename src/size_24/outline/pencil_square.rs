use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M16%2E862%204%2E487l1%2E687%2D1%2E688a1%2E875%201%2E875%200%20112%2E652%202%2E652L10%2E582%2016%2E07a4%2E5%204%2E5%200%2001%2D1%2E897%201%2E13L6%2018l%2E8%2D2%2E685a4%2E5%204%2E5%200%20011%2E13%2D1%2E897l8%2E932%2D8%2E931zm0%200L19%2E5%207%2E125M18%2014v4%2E75A2%2E25%202%2E25%200%200115%2E75%2021H5%2E25A2%2E25%202%2E25%200%20013%2018%2E75V8%2E25A2%2E25%202%2E25%200%20015%2E25%206H10%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PencilSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10"/>
</svg>
  }
}
