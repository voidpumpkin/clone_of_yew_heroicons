use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E5%206v%2E75H5%2E513c%2D%2E96%200%2D1%2E764%2E724%2D1%2E865%201%2E679l%2D1%2E263%2012A1%2E875%201%2E875%200%20004%2E25%2022%2E5h15%2E5a1%2E875%201%2E875%200%20001%2E865%2D2%2E071l%2D1%2E263%2D12a1%2E875%201%2E875%200%2000%2D1%2E865%2D1%2E679H16%2E5V6a4%2E5%204%2E5%200%2010%2D9%200zM12%203a3%203%200%2000%2D3%203v%2E75h6V6a3%203%200%2000%2D3%2D3zm%2D3%208%2E25a3%203%200%20106%200v%2D%2E75a%2E75%2E75%200%20011%2E5%200v%2E75a4%2E5%204%2E5%200%2011%2D9%200v%2D%2E75a%2E75%2E75%200%20011%2E5%200v%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShoppingBagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M7.5 6v.75H5.513c-.96 0-1.764.724-1.865 1.679l-1.263 12A1.875 1.875 0 004.25 22.5h15.5a1.875 1.875 0 001.865-2.071l-1.263-12a1.875 1.875 0 00-1.865-1.679H16.5V6a4.5 4.5 0 10-9 0zM12 3a3 3 0 00-3 3v.75h6V6a3 3 0 00-3-3zm-3 8.25a3 3 0 106 0v-.75a.75.75 0 011.5 0v.75a4.5 4.5 0 11-9 0v-.75a.75.75 0 011.5 0v.75z" clip-rule="evenodd"/>
</svg>
  }
}