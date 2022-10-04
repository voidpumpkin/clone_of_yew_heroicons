use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M7%2E217%2010%2E907a2%2E25%202%2E25%200%20100%202%2E186m0%2D2%2E186c%2E18%2E324%2E283%2E696%2E283%201%2E093s%2D%2E103%2E77%2D%2E283%201%2E093m0%2D2%2E186l9%2E566%2D5%2E314m%2D9%2E566%207%2E5l9%2E566%205%2E314m0%200a2%2E25%202%2E25%200%20103%2E935%202%2E186%202%2E25%202%2E25%200%2000%2D3%2E935%2D2%2E186zm0%2D12%2E814a2%2E25%202%2E25%200%20103%2E933%2D2%2E185%202%2E25%202%2E25%200%2000%2D3%2E933%202%2E185z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ShareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M7.217 10.907a2.25 2.25 0 100 2.186m0-2.186c.18.324.283.696.283 1.093s-.103.77-.283 1.093m0-2.186l9.566-5.314m-9.566 7.5l9.566 5.314m0 0a2.25 2.25 0 103.935 2.186 2.25 2.25 0 00-3.935-2.186zm0-12.814a2.25 2.25 0 103.933-2.185 2.25 2.25 0 00-3.933 2.185z"/>
</svg>
  }
}