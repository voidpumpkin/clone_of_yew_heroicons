use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%202%2E25c%2D5%2E385%200%2D9%2E75%204%2E365%2D9%2E75%209%2E75s4%2E365%209%2E75%209%2E75%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75S17%2E385%202%2E25%2012%202%2E25zM9%2E624%207%2E084a%2E75%2E75%200%2000%2D1%2E248%2E832l2%2E223%203%2E334H9a%2E75%2E75%200%20000%201%2E5h2%2E25v1%2E5H9a%2E75%2E75%200%20000%201%2E5h2%2E25v1%2E5a%2E75%2E75%200%20001%2E5%200v%2D1%2E5H15a%2E75%2E75%200%20000%2D1%2E5h%2D2%2E25v%2D1%2E5H15a%2E75%2E75%200%20000%2D1%2E5h%2D1%2E599l2%2E223%2D3%2E334a%2E75%2E75%200%2010%2D1%2E248%2D%2E832L12%2010%2E648%209%2E624%207%2E084z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CurrencyYenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zM9.624 7.084a.75.75 0 00-1.248.832l2.223 3.334H9a.75.75 0 000 1.5h2.25v1.5H9a.75.75 0 000 1.5h2.25v1.5a.75.75 0 001.5 0v-1.5H15a.75.75 0 000-1.5h-2.25v-1.5H15a.75.75 0 000-1.5h-1.599l2.223-3.334a.75.75 0 10-1.248-.832L12 10.648 9.624 7.084z" clip-rule="evenodd"/>
</svg>
  }
}