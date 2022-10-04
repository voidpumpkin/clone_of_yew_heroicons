use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2E75%2010%2E5l4%2E72%2D4%2E72a%2E75%2E75%200%20011%2E28%2E53v11%2E38a%2E75%2E75%200%2001%2D1%2E28%2E53l%2D4%2E72%2D4%2E72M12%2018%2E75H4%2E5a2%2E25%202%2E25%200%2001%2D2%2E25%2D2%2E25V9m12%2E841%209%2E091L16%2E5%2019%2E5m%2D1%2E409%2D1%2E409c%2E407%2D%2E407%2E659%2D%2E97%2E659%2D1%2E591v%2D9a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25h%2D9c%2D%2E621%200%2D1%2E184%2E252%2D1%2E591%2E659m12%2E182%2012%2E182L2%2E909%205%2E909M1%2E5%204%2E5l1%2E409%201%2E409%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn VideoCameraSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M12 18.75H4.5a2.25 2.25 0 01-2.25-2.25V9m12.841 9.091L16.5 19.5m-1.409-1.409c.407-.407.659-.97.659-1.591v-9a2.25 2.25 0 00-2.25-2.25h-9c-.621 0-1.184.252-1.591.659m12.182 12.182L2.909 5.909M1.5 4.5l1.409 1.409"/>
</svg>
  }
}