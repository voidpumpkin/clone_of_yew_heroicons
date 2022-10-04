use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M8%2E5%203%2E528v4%2E644c0%20%2E729%2D%2E29%201%2E428%2D%2E805%201%2E944l%2D1%2E217%201%2E216a8%2E75%208%2E75%200%20013%2E55%2E621l%2E502%2E201a7%2E25%207%2E25%200%20004%2E178%2E365l%2D2%2E403%2D2%2E403a2%2E75%202%2E75%200%2001%2D%2E805%2D1%2E944V3%2E528a40%2E205%2040%2E205%200%2000%2D3%200zm4%2E5%2E084l%2E19%2E015a%2E75%2E75%200%2010%2E12%2D1%2E495%2041%2E364%2041%2E364%200%2000%2D6%2E62%200%20%2E75%2E75%200%2000%2E12%201%2E495L7%203%2E612v4%2E56c0%20%2E331%2D%2E132%2E649%2D%2E366%2E883L2%2E6%2013%2E09c%2D1%2E496%201%2E496%2D%2E817%204%2E15%201%2E403%204%2E475C5%2E961%2017%2E852%207%2E963%2018%2010%2018s4%2E039%2D%2E148%205%2E997%2D%2E436c2%2E22%2D%2E325%202%2E9%2D2%2E979%201%2E403%2D4%2E475l%2D4%2E034%2D4%2E034A1%2E25%201%2E25%200%200113%208%2E172v%2D4%2E56z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BeakerIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M8.5 3.528v4.644c0 .729-.29 1.428-.805 1.944l-1.217 1.216a8.75 8.75 0 013.55.621l.502.201a7.25 7.25 0 004.178.365l-2.403-2.403a2.75 2.75 0 01-.805-1.944V3.528a40.205 40.205 0 00-3 0zm4.5.084l.19.015a.75.75 0 10.12-1.495 41.364 41.364 0 00-6.62 0 .75.75 0 00.12 1.495L7 3.612v4.56c0 .331-.132.649-.366.883L2.6 13.09c-1.496 1.496-.817 4.15 1.403 4.475C5.961 17.852 7.963 18 10 18s4.039-.148 5.997-.436c2.22-.325 2.9-2.979 1.403-4.475l-4.034-4.034A1.25 1.25 0 0113 8.172v-4.56z" clip-rule="evenodd"/>
</svg>
  }
}
