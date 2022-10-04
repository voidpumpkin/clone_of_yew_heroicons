use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M14%2E121%207%2E629A3%203%200%20009%2E017%209%2E43c%2D%2E023%2E212%2D%2E002%2E425%2E028%2E636l%2E506%203%2E541a4%2E5%204%2E5%200%2001%2D%2E43%202%2E65L9%2016%2E5l1%2E539%2D%2E513a2%2E25%202%2E25%200%20011%2E422%200l%2E655%2E218a2%2E25%202%2E25%200%20001%2E718%2D%2E122L15%2015%2E75M8%2E25%2012H12m9%200a9%209%200%2011%2D18%200%209%209%200%200118%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CurrencyPoundIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M14.121 7.629A3 3 0 009.017 9.43c-.023.212-.002.425.028.636l.506 3.541a4.5 4.5 0 01-.43 2.65L9 16.5l1.539-.513a2.25 2.25 0 011.422 0l.655.218a2.25 2.25 0 001.718-.122L15 15.75M8.25 12H12m9 0a9 9 0 11-18 0 9 9 0 0118 0z"/>
</svg>
  }
}
