use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2E813%2015%2E904L9%2018%2E75l%2D%2E813%2D2%2E846a4%2E5%204%2E5%200%2000%2D3%2E09%2D3%2E09L2%2E25%2012l2%2E846%2D%2E813a4%2E5%204%2E5%200%20003%2E09%2D3%2E09L9%205%2E25l%2E813%202%2E846a4%2E5%204%2E5%200%20003%2E09%203%2E09L15%2E75%2012l%2D2%2E846%2E813a4%2E5%204%2E5%200%2000%2D3%2E09%203%2E09zM18%2E259%208%2E715L18%209%2E75l%2D%2E259%2D1%2E035a3%2E375%203%2E375%200%2000%2D2%2E455%2D2%2E456L14%2E25%206l1%2E036%2D%2E259a3%2E375%203%2E375%200%20002%2E455%2D2%2E456L18%202%2E25l%2E259%201%2E035a3%2E375%203%2E375%200%20002%2E456%202%2E456L21%2E75%206l%2D1%2E035%2E259a3%2E375%203%2E375%200%2000%2D2%2E456%202%2E456zM16%2E894%2020%2E567L16%2E5%2021%2E75l%2D%2E394%2D1%2E183a2%2E25%202%2E25%200%2000%2D1%2E423%2D1%2E423L13%2E5%2018%2E75l1%2E183%2D%2E394a2%2E25%202%2E25%200%20001%2E423%2D1%2E423l%2E394%2D1%2E183%2E394%201%2E183a2%2E25%202%2E25%200%20001%2E423%201%2E423l1%2E183%2E394%2D1%2E183%2E394a2%2E25%202%2E25%200%2000%2D1%2E423%201%2E423z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SparklesIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z"/>
</svg>
  }
}