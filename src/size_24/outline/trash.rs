use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M14%2E74%209l%2D%2E346%209m%2D4%2E788%200L9%2E26%209m9%2E968%2D3%2E21c%2E342%2E052%2E682%2E107%201%2E022%2E166m%2D1%2E022%2D%2E165L18%2E16%2019%2E673a2%2E25%202%2E25%200%2001%2D2%2E244%202%2E077H8%2E084a2%2E25%202%2E25%200%2001%2D2%2E244%2D2%2E077L4%2E772%205%2E79m14%2E456%200a48%2E108%2048%2E108%200%2000%2D3%2E478%2D%2E397m%2D12%20%2E562c%2E34%2D%2E059%2E68%2D%2E114%201%2E022%2D%2E165m0%200a48%2E11%2048%2E11%200%20013%2E478%2D%2E397m7%2E5%200v%2D%2E916c0%2D1%2E18%2D%2E91%2D2%2E164%2D2%2E09%2D2%2E201a51%2E964%2051%2E964%200%2000%2D3%2E32%200c%2D1%2E18%2E037%2D2%2E09%201%2E022%2D2%2E09%202%2E201v%2E916m7%2E5%200a48%2E667%2048%2E667%200%2000%2D7%2E5%200%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TrashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0"/>
</svg>
  }
}
