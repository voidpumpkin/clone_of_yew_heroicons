use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2018a8%208%200%20100%2D16%208%208%200%20000%2016zM6%205%2E75A%2E75%2E75%200%20016%2E75%205h6%2E5a%2E75%2E75%200%20010%201%2E5h%2D2%2E127c%2E4%2E5%2E683%201%2E096%2E807%201%2E75h1%2E32a%2E75%2E75%200%20010%201%2E5h%2D1%2E32a4%2E003%204%2E003%200%2001%2D3%2E404%203%2E216l1%2E754%201%2E754a%2E75%2E75%200%2001%2D1%2E06%201%2E06l%2D3%2D3a%2E75%2E75%200%2001%2E53%2D1%2E28H8c1%2E12%200%202%2E067%2D%2E736%202%2E386%2D1%2E75H6%2E75a%2E75%2E75%200%20010%2D1%2E5h3%2E636A2%2E501%202%2E501%200%20008%206%2E5H6%2E75A%2E75%2E75%200%20016%205%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CurrencyRupeeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM6 5.75A.75.75 0 016.75 5h6.5a.75.75 0 010 1.5h-2.127c.4.5.683 1.096.807 1.75h1.32a.75.75 0 010 1.5h-1.32a4.003 4.003 0 01-3.404 3.216l1.754 1.754a.75.75 0 01-1.06 1.06l-3-3a.75.75 0 01.53-1.28H8c1.12 0 2.067-.736 2.386-1.75H6.75a.75.75 0 010-1.5h3.636A2.501 2.501 0 008 6.5H6.75A.75.75 0 016 5.75z" clip-rule="evenodd"/>
</svg>
  }
}