use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%2E493%202%2E853a%2E75%2E75%200%2000%2D1%2E486%2D%2E205L7%2E545%206H4%2E198a%2E75%2E75%200%20000%201%2E5h3%2E14l%2D%2E69%205H3%2E302a%2E75%2E75%200%20000%201%2E5h3%2E14l%2D%2E435%203%2E148a%2E75%2E75%200%20001%2E486%2E205L7%2E955%2014h2%2E986l%2D%2E434%203%2E148a%2E75%2E75%200%20001%2E486%2E205L12%2E456%2014h3%2E346a%2E75%2E75%200%20000%2D1%2E5h%2D3%2E14l%2E69%2D5h3%2E346a%2E75%2E75%200%20000%2D1%2E5h%2D3%2E14l%2E435%2D3%2E147a%2E75%2E75%200%2000%2D1%2E486%2D%2E205L12%2E045%206H9%2E059l%2E434%2D3%2E147zM8%2E852%207%2E5l%2D%2E69%205h2%2E986l%2E69%2D5H8%2E852z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HashtagIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M9.493 2.853a.75.75 0 00-1.486-.205L7.545 6H4.198a.75.75 0 000 1.5h3.14l-.69 5H3.302a.75.75 0 000 1.5h3.14l-.435 3.148a.75.75 0 001.486.205L7.955 14h2.986l-.434 3.148a.75.75 0 001.486.205L12.456 14h3.346a.75.75 0 000-1.5h-3.14l.69-5h3.346a.75.75 0 000-1.5h-3.14l.435-3.147a.75.75 0 00-1.486-.205L12.045 6H9.059l.434-3.147zM8.852 7.5l-.69 5h2.986l.69-5H8.852z" clip-rule="evenodd"/>
</svg>
  }
}
