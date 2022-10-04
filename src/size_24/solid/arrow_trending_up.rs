use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M15%2E22%206%2E268a%2E75%2E75%200%2001%2E968%2D%2E432l5%2E942%202%2E28a%2E75%2E75%200%2001%2E431%2E97l%2D2%2E28%205%2E941a%2E75%2E75%200%2011%2D1%2E4%2D%2E537l1%2E63%2D4%2E251%2D1%2E086%2E483a11%2E2%2011%2E2%200%2000%2D5%2E45%205%2E174%2E75%2E75%200%2001%2D1%2E199%2E19L9%2012%2E31l%2D6%2E22%206%2E22a%2E75%2E75%200%2011%2D1%2E06%2D1%2E06l6%2E75%2D6%2E75a%2E75%2E75%200%20011%2E06%200l3%2E606%203%2E605a12%2E694%2012%2E694%200%20015%2E68%2D4%2E973l1%2E086%2D%2E484%2D4%2E251%2D1%2E631a%2E75%2E75%200%2001%2D%2E432%2D%2E97z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowTrendingUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M15.22 6.268a.75.75 0 01.968-.432l5.942 2.28a.75.75 0 01.431.97l-2.28 5.941a.75.75 0 11-1.4-.537l1.63-4.251-1.086.483a11.2 11.2 0 00-5.45 5.174.75.75 0 01-1.199.19L9 12.31l-6.22 6.22a.75.75 0 11-1.06-1.06l6.75-6.75a.75.75 0 011.06 0l3.606 3.605a12.694 12.694 0 015.68-4.973l1.086-.484-4.251-1.631a.75.75 0 01-.432-.97z" clip-rule="evenodd"/>
</svg>
  }
}
