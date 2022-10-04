use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%2E5%203%2E75a3%203%200%2000%2D3%203v10%2E5a3%203%200%20003%203h15a3%203%200%20003%2D3V6%2E75a3%203%200%2000%2D3%2D3h%2D15zm9%204%2E5a%2E75%2E75%200%2000%2D1%2E5%200v7%2E5a%2E75%2E75%200%20001%2E5%200v%2D7%2E5zm1%2E5%200a%2E75%2E75%200%2001%2E75%2D%2E75h3a%2E75%2E75%200%20010%201%2E5H16%2E5v2%2E25H18a%2E75%2E75%200%20010%201%2E5h%2D1%2E5v3a%2E75%2E75%200%2001%2D1%2E5%200v%2D7%2E5zM6%2E636%209%2E78c%2E404%2D%2E575%2E867%2D%2E78%201%2E25%2D%2E78s%2E846%2E205%201%2E25%2E78a%2E75%2E75%200%20001%2E228%2D%2E863C9%2E738%208%2E027%208%2E853%207%2E5%207%2E886%207%2E5c%2D%2E966%200%2D1%2E852%2E527%2D2%2E478%201%2E417%2D%2E62%2E882%2D%2E908%202%2D%2E908%203%2E083%200%201%2E083%2E288%202%2E201%2E909%203%2E083%2E625%2E89%201%2E51%201%2E417%202%2E477%201%2E417%2E967%200%201%2E852%2D%2E527%202%2E478%2D1%2E417a%2E75%2E75%200%2000%2E136%2D%2E431V12a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D1%2E5a%2E75%2E75%200%20000%201%2E5H9v1%2E648c%2D%2E37%2E44%2D%2E774%2E602%2D1%2E114%2E602%2D%2E383%200%2D%2E846%2D%2E205%2D1%2E25%2D%2E78C6%2E226%2013%2E638%206%2012%2E837%206%2012c0%2D%2E837%2E226%2D1%2E638%2E636%2D2%2E22z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn GifIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M4.5 3.75a3 3 0 00-3 3v10.5a3 3 0 003 3h15a3 3 0 003-3V6.75a3 3 0 00-3-3h-15zm9 4.5a.75.75 0 00-1.5 0v7.5a.75.75 0 001.5 0v-7.5zm1.5 0a.75.75 0 01.75-.75h3a.75.75 0 010 1.5H16.5v2.25H18a.75.75 0 010 1.5h-1.5v3a.75.75 0 01-1.5 0v-7.5zM6.636 9.78c.404-.575.867-.78 1.25-.78s.846.205 1.25.78a.75.75 0 001.228-.863C9.738 8.027 8.853 7.5 7.886 7.5c-.966 0-1.852.527-2.478 1.417-.62.882-.908 2-.908 3.083 0 1.083.288 2.201.909 3.083.625.89 1.51 1.417 2.477 1.417.967 0 1.852-.527 2.478-1.417a.75.75 0 00.136-.431V12a.75.75 0 00-.75-.75h-1.5a.75.75 0 000 1.5H9v1.648c-.37.44-.774.602-1.114.602-.383 0-.846-.205-1.25-.78C6.226 13.638 6 12.837 6 12c0-.837.226-1.638.636-2.22z" clip-rule="evenodd"/>
</svg>
  }
}