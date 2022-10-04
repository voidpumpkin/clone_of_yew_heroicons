use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E5%2017a4%2E5%204%2E5%200%2001%2D1%2E44%2D8%2E765%204%2E5%204%2E5%200%20018%2E302%2D3%2E046%203%2E5%203%2E5%200%20014%2E504%204%2E272A4%204%200%200115%2017H5%2E5zm5%2E25%2D9%2E25a%2E75%2E75%200%2000%2D1%2E5%200v4%2E59l%2D1%2E95%2D2%2E1a%2E75%2E75%200%2010%2D1%2E1%201%2E02l3%2E25%203%2E5a%2E75%2E75%200%20001%2E1%200l3%2E25%2D3%2E5a%2E75%2E75%200%2010%2D1%2E1%2D1%2E02l%2D1%2E95%202%2E1V7%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CloudArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.5 17a4.5 4.5 0 01-1.44-8.765 4.5 4.5 0 018.302-3.046 3.5 3.5 0 014.504 4.272A4 4 0 0115 17H5.5zm5.25-9.25a.75.75 0 00-1.5 0v4.59l-1.95-2.1a.75.75 0 10-1.1 1.02l3.25 3.5a.75.75 0 001.1 0l3.25-3.5a.75.75 0 10-1.1-1.02l-1.95 2.1V7.75z" clip-rule="evenodd"/>
</svg>
  }
}
