use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E625%2012a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200zm0%200H8%2E25m4%2E125%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200zm0%200H12m4%2E125%200a%2E375%2E375%200%2011%2D%2E75%200%20%2E375%2E375%200%2001%2E75%200zm0%200h%2D%2E375M21%2012c0%204%2E556%2D4%2E03%208%2E25%2D9%208%2E25a9%2E764%209%2E764%200%2001%2D2%2E555%2D%2E337A5%2E972%205%2E972%200%20015%2E41%2020%2E97a5%2E969%205%2E969%200%2001%2D%2E474%2D%2E065%204%2E48%204%2E48%200%2000%2E978%2D2%2E025c%2E09%2D%2E457%2D%2E133%2D%2E901%2D%2E467%2D1%2E226C3%2E93%2016%2E178%203%2014%2E189%203%2012c0%2D4%2E556%204%2E03%2D8%2E25%209%2D8%2E25s9%203%2E694%209%208%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleOvalLeftEllipsisIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.625 12a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H8.25m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0H12m4.125 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 01-2.555-.337A5.972 5.972 0 015.41 20.97a5.969 5.969 0 01-.474-.065 4.48 4.48 0 00.978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25z"/>
</svg>
  }
}
