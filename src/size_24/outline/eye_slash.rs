use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%2E98%208%2E223A10%2E477%2010%2E477%200%20001%2E934%2012C3%2E226%2016%2E338%207%2E244%2019%2E5%2012%2019%2E5c%2E993%200%201%2E953%2D%2E138%202%2E863%2D%2E395M6%2E228%206%2E228A10%2E45%2010%2E45%200%200112%204%2E5c4%2E756%200%208%2E773%203%2E162%2010%2E065%207%2E498a10%2E523%2010%2E523%200%2001%2D4%2E293%205%2E774M6%2E228%206%2E228L3%203m3%2E228%203%2E228l3%2E65%203%2E65m7%2E894%207%2E894L21%2021m%2D3%2E228%2D3%2E228l%2D3%2E65%2D3%2E65m0%200a3%203%200%2010%2D4%2E243%2D4%2E243m4%2E242%204%2E242L9%2E88%209%2E88%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EyeSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.773 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.242 4.242L9.88 9.88"/>
</svg>
  }
}