use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M15%2E312%2011%2E424a5%2E5%205%2E5%200%2001%2D9%2E201%202%2E466l%2D%2E312%2D%2E311h2%2E433a%2E75%2E75%200%20000%2D1%2E5H3%2E989a%2E75%2E75%200%2000%2D%2E75%2E75v4%2E242a%2E75%2E75%200%20001%2E5%200v%2D2%2E43l%2E31%2E31a7%207%200%200011%2E712%2D3%2E138%2E75%2E75%200%2000%2D1%2E449%2D%2E39zm1%2E23%2D3%2E723a%2E75%2E75%200%2000%2E219%2D%2E53V2%2E929a%2E75%2E75%200%2000%2D1%2E5%200V5%2E36l%2D%2E31%2D%2E31A7%207%200%20003%2E239%208%2E188a%2E75%2E75%200%20101%2E448%2E389A5%2E5%205%2E5%200%200113%2E89%206%2E11l%2E311%2E31h%2D2%2E432a%2E75%2E75%200%20000%201%2E5h4%2E243a%2E75%2E75%200%2000%2E53%2D%2E219z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowPathIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M15.312 11.424a5.5 5.5 0 01-9.201 2.466l-.312-.311h2.433a.75.75 0 000-1.5H3.989a.75.75 0 00-.75.75v4.242a.75.75 0 001.5 0v-2.43l.31.31a7 7 0 0011.712-3.138.75.75 0 00-1.449-.39zm1.23-3.723a.75.75 0 00.219-.53V2.929a.75.75 0 00-1.5 0V5.36l-.31-.31A7 7 0 003.239 8.188a.75.75 0 101.448.389A5.5 5.5 0 0113.89 6.11l.311.31h-2.432a.75.75 0 000 1.5h4.243a.75.75 0 00.53-.219z" clip-rule="evenodd"/>
</svg>
  }
}
