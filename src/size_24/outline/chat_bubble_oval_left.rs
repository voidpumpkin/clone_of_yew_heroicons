use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%2020%2E25c4%2E97%200%209%2D3%2E694%209%2D8%2E25s%2D4%2E03%2D8%2E25%2D9%2D8%2E25S3%207%2E444%203%2012c0%202%2E104%2E859%204%2E023%202%2E273%205%2E48%2E432%2E447%2E74%201%2E04%2E586%201%2E641a4%2E483%204%2E483%200%2001%2D%2E923%201%2E785A5%2E969%205%2E969%200%20006%2021c1%2E282%200%202%2E47%2D%2E402%203%2E445%2D1%2E087%2E81%2E22%201%2E668%2E337%202%2E555%2E337z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleOvalLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 20.25c4.97 0 9-3.694 9-8.25s-4.03-8.25-9-8.25S3 7.444 3 12c0 2.104.859 4.023 2.273 5.48.432.447.74 1.04.586 1.641a4.483 4.483 0 01-.923 1.785A5.969 5.969 0 006 21c1.282 0 2.47-.402 3.445-1.087.81.22 1.668.337 2.555.337z"/>
</svg>
  }
}