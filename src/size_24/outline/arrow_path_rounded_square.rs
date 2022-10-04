use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M19%2E5%2012c0%2D1%2E232%2D%2E046%2D2%2E453%2D%2E138%2D3%2E662a4%2E006%204%2E006%200%2000%2D3%2E7%2D3%2E7%2048%2E678%2048%2E678%200%2000%2D7%2E324%200%204%2E006%204%2E006%200%2000%2D3%2E7%203%2E7c%2D%2E017%2E22%2D%2E032%2E441%2D%2E046%2E662M19%2E5%2012l3%2D3m%2D3%203l%2D3%2D3m%2D12%203c0%201%2E232%2E046%202%2E453%2E138%203%2E662a4%2E006%204%2E006%200%20003%2E7%203%2E7%2048%2E656%2048%2E656%200%20007%2E324%200%204%2E006%204%2E006%200%20003%2E7%2D3%2E7c%2E017%2D%2E22%2E032%2D%2E441%2E046%2D%2E662M4%2E5%2012l3%203m%2D3%2D3l%2D3%203%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowPathRoundedSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 00-3.7-3.7 48.678 48.678 0 00-7.324 0 4.006 4.006 0 00-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3l-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 003.7 3.7 48.656 48.656 0 007.324 0 4.006 4.006 0 003.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3l-3 3"/>
</svg>
  }
}
