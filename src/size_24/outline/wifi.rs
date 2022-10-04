use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E288%2015%2E038a5%2E25%205%2E25%200%20017%2E424%200M5%2E106%2011%2E856c3%2E807%2D3%2E808%209%2E98%2D3%2E808%2013%2E788%200M1%2E924%208%2E674c5%2E565%2D5%2E565%2014%2E587%2D5%2E565%2020%2E152%200M12%2E53%2018%2E22l%2D%2E53%2E53%2D%2E53%2D%2E53a%2E75%2E75%200%20011%2E06%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn WifiIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.288 15.038a5.25 5.25 0 017.424 0M5.106 11.856c3.807-3.808 9.98-3.808 13.788 0M1.924 8.674c5.565-5.565 14.587-5.565 20.152 0M12.53 18.22l-.53.53-.53-.53a.75.75 0 011.06 0z"/>
</svg>
  }
}