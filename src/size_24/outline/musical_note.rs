use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%209l10%2E5%2D3m0%206%2E553v3%2E75a2%2E25%202%2E25%200%2001%2D1%2E632%202%2E163l%2D1%2E32%2E377a1%2E803%201%2E803%200%2011%2D%2E99%2D3%2E467l2%2E31%2D%2E66a2%2E25%202%2E25%200%20001%2E632%2D2%2E163zm0%200V2%2E25L9%205%2E25v10%2E303m0%200v3%2E75a2%2E25%202%2E25%200%2001%2D1%2E632%202%2E163l%2D1%2E32%2E377a1%2E803%201%2E803%200%2001%2D%2E99%2D3%2E467l2%2E31%2D%2E66A2%2E25%202%2E25%200%20009%2015%2E553z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MusicalNoteIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 9l10.5-3m0 6.553v3.75a2.25 2.25 0 01-1.632 2.163l-1.32.377a1.803 1.803 0 11-.99-3.467l2.31-.66a2.25 2.25 0 001.632-2.163zm0 0V2.25L9 5.25v10.303m0 0v3.75a2.25 2.25 0 01-1.632 2.163l-1.32.377a1.803 1.803 0 01-.99-3.467l2.31-.66A2.25 2.25 0 009 15.553z"/>
</svg>
  }
}
