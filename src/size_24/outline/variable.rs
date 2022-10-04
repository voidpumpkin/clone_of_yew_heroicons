use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M4%2E745%203A23%2E933%2023%2E933%200%20003%2012c0%203%2E183%2E62%206%2E22%201%2E745%209M19%2E5%203c%2E967%202%2E78%201%2E5%205%2E817%201%2E5%209s%2D%2E533%206%2E22%2D1%2E5%209M8%2E25%208%2E885l1%2E444%2D%2E89a%2E75%2E75%200%20011%2E105%2E402l2%2E402%207%2E206a%2E75%2E75%200%20001%2E104%2E401l1%2E445%2D%2E889m%2D8%2E25%2E75l%2E213%2E09a1%2E687%201%2E687%200%20002%2E062%2D%2E617l4%2E45%2D6%2E676a1%2E688%201%2E688%200%20012%2E062%2D%2E618l%2E213%2E09%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn VariableIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M4.745 3A23.933 23.933 0 003 12c0 3.183.62 6.22 1.745 9M19.5 3c.967 2.78 1.5 5.817 1.5 9s-.533 6.22-1.5 9M8.25 8.885l1.444-.89a.75.75 0 011.105.402l2.402 7.206a.75.75 0 001.104.401l1.445-.889m-8.25.75l.213.09a1.687 1.687 0 002.062-.617l4.45-6.676a1.688 1.688 0 012.062-.618l.213.09"/>
</svg>
  }
}