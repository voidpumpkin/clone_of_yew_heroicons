use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E106%206%2E447A2%202%200%20001%208%2E237V16a2%202%200%20002%202h14a2%202%200%20002%2D2V8%2E236a2%202%200%2000%2D1%2E106%2D1%2E789l%2D7%2D3%2E5a2%202%200%2000%2D1%2E788%200l%2D7%203%2E5zm1%2E48%204%2E007a%2E75%2E75%200%2000%2D%2E671%201%2E342l5%2E855%202%2E928a2%2E75%202%2E75%200%20002%2E46%200l5%2E852%2D2%2E926a%2E75%2E75%200%2010%2D%2E67%2D1%2E342l%2D5%2E853%202%2E926a1%2E25%201%2E25%200%2001%2D1%2E118%200l%2D5%2E856%2D2%2E928z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EnvelopeOpenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.106 6.447A2 2 0 001 8.237V16a2 2 0 002 2h14a2 2 0 002-2V8.236a2 2 0 00-1.106-1.789l-7-3.5a2 2 0 00-1.788 0l-7 3.5zm1.48 4.007a.75.75 0 00-.671 1.342l5.855 2.928a2.75 2.75 0 002.46 0l5.852-2.926a.75.75 0 10-.67-1.342l-5.853 2.926a1.25 1.25 0 01-1.118 0l-5.856-2.928z" clip-rule="evenodd"/>
</svg>
  }
}
