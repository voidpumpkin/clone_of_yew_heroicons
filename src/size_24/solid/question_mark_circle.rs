use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%2012c0%2D5%2E385%204%2E365%2D9%2E75%209%2E75%2D9%2E75s9%2E75%204%2E365%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75%209%2E75S2%2E25%2017%2E385%202%2E25%2012zm11%2E378%2D3%2E917c%2D%2E89%2D%2E777%2D2%2E366%2D%2E777%2D3%2E255%200a%2E75%2E75%200%2001%2D%2E988%2D1%2E129c1%2E454%2D1%2E272%203%2E776%2D1%2E272%205%2E23%200%201%2E513%201%2E324%201%2E513%203%2E518%200%204%2E842a3%2E75%203%2E75%200%2001%2D%2E837%2E552c%2D%2E676%2E328%2D1%2E028%2E774%2D1%2E028%201%2E152v%2E75a%2E75%2E75%200%2001%2D1%2E5%200v%2D%2E75c0%2D1%2E279%201%2E06%2D2%2E107%201%2E875%2D2%2E502%2E182%2D%2E088%2E351%2D%2E199%2E503%2D%2E331%2E83%2D%2E727%2E83%2D1%2E857%200%2D2%2E584zM12%2018a%2E75%2E75%200%20100%2D1%2E5%2E75%2E75%200%20000%201%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn QuestionMarkCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm11.378-3.917c-.89-.777-2.366-.777-3.255 0a.75.75 0 01-.988-1.129c1.454-1.272 3.776-1.272 5.23 0 1.513 1.324 1.513 3.518 0 4.842a3.75 3.75 0 01-.837.552c-.676.328-1.028.774-1.028 1.152v.75a.75.75 0 01-1.5 0v-.75c0-1.279 1.06-2.107 1.875-2.502.182-.088.351-.199.503-.331.83-.727.83-1.857 0-2.584zM12 18a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd"/>
</svg>
  }
}