use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E171%204%2E146l1%2E947%202%2E466a3%2E514%203%2E514%200%20011%2E764%200l1%2E947%2D2%2E466a6%2E52%206%2E52%200%2000%2D5%2E658%200zm8%2E683%203%2E025l%2D2%2E466%201%2E947c%2E15%2E578%2E15%201%2E186%200%201%2E764l2%2E466%201%2E947a6%2E52%206%2E52%200%20000%2D5%2E658zm%2D3%2E025%208%2E683l%2D1%2E947%2D2%2E466c%2D%2E578%2E15%2D1%2E186%2E15%2D1%2E764%200l%2D1%2E947%202%2E466a6%2E52%206%2E52%200%20005%2E658%200zM4%2E146%2012%2E83l2%2E466%2D1%2E947a3%2E514%203%2E514%200%20010%2D1%2E764L4%2E146%207%2E171a6%2E52%206%2E52%200%20000%205%2E658zM5%2E63%203%2E297a8%2E01%208%2E01%200%20018%2E738%200%208%2E031%208%2E031%200%20012%2E334%202%2E334%208%2E01%208%2E01%200%20010%208%2E738%208%2E033%208%2E033%200%2001%2D2%2E334%202%2E334%208%2E01%208%2E01%200%2001%2D8%2E738%200%208%2E032%208%2E032%200%2001%2D2%2E334%2D2%2E334%208%2E01%208%2E01%200%20010%2D8%2E738A8%2E03%208%2E03%200%20015%2E63%203%2E297zm5%2E198%204%2E882a2%2E008%202%2E008%200%2000%2D2%2E243%2E407%201%2E994%201%2E994%200%2000%2D%2E407%202%2E243%201%2E993%201%2E993%200%2000%2E992%2E992%202%2E008%202%2E008%200%20002%2E243%2D%2E407c%2E176%2D%2E175%2E31%2D%2E374%2E407%2D%2E585a2%2E008%202%2E008%200%2000%2D%2E407%2D2%2E243%201%2E993%201%2E993%200%2000%2D%2E585%2D%2E407z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LifebuoyIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M7.171 4.146l1.947 2.466a3.514 3.514 0 011.764 0l1.947-2.466a6.52 6.52 0 00-5.658 0zm8.683 3.025l-2.466 1.947c.15.578.15 1.186 0 1.764l2.466 1.947a6.52 6.52 0 000-5.658zm-3.025 8.683l-1.947-2.466c-.578.15-1.186.15-1.764 0l-1.947 2.466a6.52 6.52 0 005.658 0zM4.146 12.83l2.466-1.947a3.514 3.514 0 010-1.764L4.146 7.171a6.52 6.52 0 000 5.658zM5.63 3.297a8.01 8.01 0 018.738 0 8.031 8.031 0 012.334 2.334 8.01 8.01 0 010 8.738 8.033 8.033 0 01-2.334 2.334 8.01 8.01 0 01-8.738 0 8.032 8.032 0 01-2.334-2.334 8.01 8.01 0 010-8.738A8.03 8.03 0 015.63 3.297zm5.198 4.882a2.008 2.008 0 00-2.243.407 1.994 1.994 0 00-.407 2.243 1.993 1.993 0 00.992.992 2.008 2.008 0 002.243-.407c.176-.175.31-.374.407-.585a2.008 2.008 0 00-.407-2.243 1.993 1.993 0 00-.585-.407z" clip-rule="evenodd"/>
</svg>
  }
}
