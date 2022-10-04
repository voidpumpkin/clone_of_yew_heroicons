use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M11%2E48%203%2E499a%2E562%2E562%200%20011%2E04%200l2%2E125%205%2E111a%2E563%2E563%200%2000%2E475%2E345l5%2E518%2E442c%2E499%2E04%2E701%2E663%2E321%2E988l%2D4%2E204%203%2E602a%2E563%2E563%200%2000%2D%2E182%2E557l1%2E285%205%2E385a%2E562%2E562%200%2001%2D%2E84%2E61l%2D4%2E725%2D2%2E885a%2E563%2E563%200%2000%2D%2E586%200L6%2E982%2020%2E54a%2E562%2E562%200%2001%2D%2E84%2D%2E61l1%2E285%2D5%2E386a%2E562%2E562%200%2000%2D%2E182%2D%2E557l%2D4%2E204%2D3%2E602a%2E563%2E563%200%2001%2E321%2D%2E988l5%2E518%2D%2E442a%2E563%2E563%200%2000%2E475%2D%2E345L11%2E48%203%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn StarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"/>
</svg>
  }
}