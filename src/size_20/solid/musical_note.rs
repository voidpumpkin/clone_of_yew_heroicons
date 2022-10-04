use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M17%2E721%201%2E599a%2E75%2E75%200%2001%2E279%2E584v11%2E29a2%2E25%202%2E25%200%2001%2D1%2E774%202%2E198l%2D2%2E041%2E442a2%2E216%202%2E216%200%2001%2D%2E938%2D4%2E333l2%2E662%2D%2E576a%2E75%2E75%200%2000%2E591%2D%2E734V6%2E112l%2D8%201%2E73v7%2E684a2%2E25%202%2E25%200%2001%2D1%2E774%202%2E2l%2D2%2E042%2E44a2%2E216%202%2E216%200%2011%2D%2E935%2D4%2E33l2%2E659%2D%2E574A%2E75%2E75%200%20007%2012%2E53V4%2E237a%2E75%2E75%200%2001%2E591%2D%2E733l9%2E5%2D2%2E054a%2E75%2E75%200%2001%2E63%2E149z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MusicalNoteIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M17.721 1.599a.75.75 0 01.279.584v11.29a2.25 2.25 0 01-1.774 2.198l-2.041.442a2.216 2.216 0 01-.938-4.333l2.662-.576a.75.75 0 00.591-.734V6.112l-8 1.73v7.684a2.25 2.25 0 01-1.774 2.2l-2.042.44a2.216 2.216 0 11-.935-4.33l2.659-.574A.75.75 0 007 12.53V4.237a.75.75 0 01.591-.733l9.5-2.054a.75.75 0 01.63.149z" clip-rule="evenodd"/>
</svg>
  }
}