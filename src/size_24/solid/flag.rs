use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%202%2E25a%2E75%2E75%200%2001%2E75%2E75v%2E54l1%2E838%2D%2E46a9%2E75%209%2E75%200%20016%2E725%2E738l%2E108%2E054a8%2E25%208%2E25%200%20005%2E58%2E652l3%2E109%2D%2E732a%2E75%2E75%200%2001%2E917%2E81%2047%2E784%2047%2E784%200%2000%2E005%2010%2E337%2E75%2E75%200%2001%2D%2E574%2E812l%2D3%2E114%2E733a9%2E75%209%2E75%200%2001%2D6%2E594%2D%2E77l%2D%2E108%2D%2E054a8%2E25%208%2E25%200%2000%2D5%2E69%2D%2E625l%2D2%2E202%2E55V21a%2E75%2E75%200%2001%2D1%2E5%200V3A%2E75%2E75%200%20013%202%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FlagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 2.25a.75.75 0 01.75.75v.54l1.838-.46a9.75 9.75 0 016.725.738l.108.054a8.25 8.25 0 005.58.652l3.109-.732a.75.75 0 01.917.81 47.784 47.784 0 00.005 10.337.75.75 0 01-.574.812l-3.114.733a9.75 9.75 0 01-6.594-.77l-.108-.054a8.25 8.25 0 00-5.69-.625l-2.202.55V21a.75.75 0 01-1.5 0V3A.75.75 0 013 2.25z" clip-rule="evenodd"/>
</svg>
  }
}