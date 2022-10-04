use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%2E912%203a3%203%200%2000%2D2%2E868%202%2E118l%2D2%2E411%207%2E838a3%203%200%2000%2D%2E133%2E882V18a3%203%200%20003%203h15a3%203%200%20003%2D3v%2D4%2E162c0%2D%2E299%2D%2E045%2D%2E596%2D%2E133%2D%2E882l%2D2%2E412%2D7%2E838A3%203%200%200017%2E088%203H6%2E912zm13%2E823%209%2E75l%2D2%2E213%2D7%2E191A1%2E5%201%2E5%200%200017%2E088%204%2E5H6%2E912a1%2E5%201%2E5%200%2000%2D1%2E434%201%2E059L3%2E265%2012%2E75H6%2E11a3%203%200%20012%2E684%201%2E658l%2E256%2E513a1%2E5%201%2E5%200%20001%2E342%2E829h3%2E218a1%2E5%201%2E5%200%20001%2E342%2D%2E83l%2E256%2D%2E512a3%203%200%20012%2E684%2D1%2E658h2%2E844z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn InboxIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6.912 3a3 3 0 00-2.868 2.118l-2.411 7.838a3 3 0 00-.133.882V18a3 3 0 003 3h15a3 3 0 003-3v-4.162c0-.299-.045-.596-.133-.882l-2.412-7.838A3 3 0 0017.088 3H6.912zm13.823 9.75l-2.213-7.191A1.5 1.5 0 0017.088 4.5H6.912a1.5 1.5 0 00-1.434 1.059L3.265 12.75H6.11a3 3 0 012.684 1.658l.256.513a1.5 1.5 0 001.342.829h3.218a1.5 1.5 0 001.342-.83l.256-.512a3 3 0 012.684-1.658h2.844z" clip-rule="evenodd"/>
</svg>
  }
}
