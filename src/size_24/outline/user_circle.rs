use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M17%2E982%2018%2E725A7%2E488%207%2E488%200%200012%2015%2E75a7%2E488%207%2E488%200%2000%2D5%2E982%202%2E975m11%2E963%200a9%209%200%2010%2D11%2E963%200m11%2E963%200A8%2E966%208%2E966%200%200112%2021a8%2E966%208%2E966%200%2001%2D5%2E982%2D2%2E275M15%209%2E75a3%203%200%2011%2D6%200%203%203%200%20016%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M17.982 18.725A7.488 7.488 0 0012 15.75a7.488 7.488 0 00-5.982 2.975m11.963 0a9 9 0 10-11.963 0m11.963 0A8.966 8.966 0 0112 21a8.966 8.966 0 01-5.982-2.275M15 9.75a3 3 0 11-6 0 3 3 0 016 0z"/>
</svg>
  }
}