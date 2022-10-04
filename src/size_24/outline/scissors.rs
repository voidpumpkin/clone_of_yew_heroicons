use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M7%2E848%208%2E25l1%2E536%2E887M7%2E848%208%2E25a3%203%200%2011%2D5%2E196%2D3%203%203%200%20015%2E196%203zm1%2E536%2E887a2%2E165%202%2E165%200%20011%2E083%201%2E839c%2E005%2E351%2E054%2E695%2E14%201%2E024M9%2E384%209%2E137l2%2E077%201%2E199M7%2E848%2015%2E75l1%2E536%2D%2E887m%2D1%2E536%2E887a3%203%200%2011%2D5%2E196%203%203%203%200%20015%2E196%2D3zm1%2E536%2D%2E887a2%2E165%202%2E165%200%20001%2E083%2D1%2E838c%2E005%2D%2E352%2E054%2D%2E695%2E14%2D1%2E025m%2D1%2E223%202%2E863l2%2E077%2D1%2E199m0%2D3%2E328a4%2E323%204%2E323%200%20012%2E068%2D1%2E379l5%2E325%2D1%2E628a4%2E5%204%2E5%200%20012%2E48%2D%2E044l%2E803%2E215%2D7%2E794%204%2E5m%2D2%2E882%2D1%2E664A4%2E331%204%2E331%200%200010%2E607%2012m3%2E736%200l7%2E794%204%2E5%2D%2E802%2E215a4%2E5%204%2E5%200%2001%2D2%2E48%2D%2E043l%2D5%2E326%2D1%2E629a4%2E324%204%2E324%200%2001%2D2%2E068%2D1%2E379M14%2E343%2012l%2D2%2E882%201%2E664%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ScissorsIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M7.848 8.25l1.536.887M7.848 8.25a3 3 0 11-5.196-3 3 3 0 015.196 3zm1.536.887a2.165 2.165 0 011.083 1.839c.005.351.054.695.14 1.024M9.384 9.137l2.077 1.199M7.848 15.75l1.536-.887m-1.536.887a3 3 0 11-5.196 3 3 3 0 015.196-3zm1.536-.887a2.165 2.165 0 001.083-1.838c.005-.352.054-.695.14-1.025m-1.223 2.863l2.077-1.199m0-3.328a4.323 4.323 0 012.068-1.379l5.325-1.628a4.5 4.5 0 012.48-.044l.803.215-7.794 4.5m-2.882-1.664A4.331 4.331 0 0010.607 12m3.736 0l7.794 4.5-.802.215a4.5 4.5 0 01-2.48-.043l-5.326-1.629a4.324 4.324 0 01-2.068-1.379M14.343 12l-2.882 1.664"/>
</svg>
  }
}
