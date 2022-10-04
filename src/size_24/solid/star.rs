use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2E788%203%2E21c%2E448%2D1%2E077%201%2E976%2D1%2E077%202%2E424%200l2%2E082%205%2E007%205%2E404%2E433c1%2E164%2E093%201%2E636%201%2E545%2E749%202%2E305l%2D4%2E117%203%2E527%201%2E257%205%2E273c%2E271%201%2E136%2D%2E964%202%2E033%2D1%2E96%201%2E425L12%2018%2E354%207%2E373%2021%2E18c%2D%2E996%2E608%2D2%2E231%2D%2E29%2D1%2E96%2D1%2E425l1%2E257%2D5%2E273%2D4%2E117%2D3%2E527c%2D%2E887%2D%2E76%2D%2E415%2D2%2E212%2E749%2D2%2E305l5%2E404%2D%2E433%202%2E082%2D5%2E006z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn StarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z" clip-rule="evenodd"/>
</svg>
  }
}
