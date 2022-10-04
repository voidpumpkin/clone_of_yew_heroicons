use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E337%2021%2E718a6%2E707%206%2E707%200%2001%2D%2E533%2D%2E074%2E75%2E75%200%2001%2D%2E44%2D1%2E223%203%2E73%203%2E73%200%2000%2E814%2D1%2E686c%2E023%2D%2E115%2D%2E022%2D%2E317%2D%2E254%2D%2E543C3%2E274%2016%2E587%202%2E25%2014%2E41%202%2E25%2012c0%2D5%2E03%204%2E428%2D9%209%2E75%2D9s9%2E75%203%2E97%209%2E75%209c0%205%2E03%2D4%2E428%209%2D9%2E75%209%2D%2E833%200%2D1%2E643%2D%2E097%2D2%2E417%2D%2E279a6%2E721%206%2E721%200%2001%2D4%2E246%2E997z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleOvalLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.337 21.718a6.707 6.707 0 01-.533-.074.75.75 0 01-.44-1.223 3.73 3.73 0 00.814-1.686c.023-.115-.022-.317-.254-.543C3.274 16.587 2.25 14.41 2.25 12c0-5.03 4.428-9 9.75-9s9.75 3.97 9.75 9c0 5.03-4.428 9-9.75 9-.833 0-1.643-.097-2.417-.279a6.721 6.721 0 01-4.246.997z" clip-rule="evenodd"/>
</svg>
  }
}
