use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M17%2E25%209%2E75L19%2E5%2012m0%200l2%2E25%202%2E25M19%2E5%2012l2%2E25%2D2%2E25M19%2E5%2012l%2D2%2E25%202%2E25m%2D10%2E5%2D6l4%2E72%2D4%2E72a%2E75%2E75%200%20011%2E28%2E531V19%2E94a%2E75%2E75%200%2001%2D1%2E28%2E53l%2D4%2E72%2D4%2E72H4%2E51c%2D%2E88%200%2D1%2E704%2D%2E506%2D1%2E938%2D1%2E354A9%2E01%209%2E01%200%20012%2E25%2012c0%2D%2E83%2E112%2D1%2E633%2E322%2D2%2E395C2%2E806%208%2E757%203%2E63%208%2E25%204%2E51%208%2E25H6%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SpeakerXMarkIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 9.75L19.5 12m0 0l2.25 2.25M19.5 12l2.25-2.25M19.5 12l-2.25 2.25m-10.5-6l4.72-4.72a.75.75 0 011.28.531V19.94a.75.75 0 01-1.28.53l-4.72-4.72H4.51c-.88 0-1.704-.506-1.938-1.354A9.01 9.01 0 012.25 12c0-.83.112-1.633.322-2.395C2.806 8.757 3.63 8.25 4.51 8.25H6.75z"/>
</svg>
  }
}