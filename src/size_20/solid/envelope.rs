use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%204a2%202%200%2000%2D2%202v1%2E161l8%2E441%204%2E221a1%2E25%201%2E25%200%20001%2E118%200L19%207%2E162V6a2%202%200%2000%2D2%2D2H3z%22%2F%3E%20%3Cpath%20d%3D%22M19%208%2E839l%2D7%2E77%203%2E885a2%2E75%202%2E75%200%2001%2D2%2E46%200L1%208%2E839V14a2%202%200%20002%202h14a2%202%200%20002%2D2V8%2E839z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EnvelopeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M3 4a2 2 0 00-2 2v1.161l8.441 4.221a1.25 1.25 0 001.118 0L19 7.162V6a2 2 0 00-2-2H3z"/>
  <path d="M19 8.839l-7.77 3.885a2.75 2.75 0 01-2.46 0L1 8.839V14a2 2 0 002 2h14a2 2 0 002-2V8.839z"/>
</svg>
  }
}