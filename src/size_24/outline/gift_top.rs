use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%203%2E75v16%2E5M2%2E25%2012h19%2E5M6%2E375%2017%2E25a4%2E875%204%2E875%200%20004%2E875%2D4%2E875V12m6%2E375%205%2E25a4%2E875%204%2E875%200%2001%2D4%2E875%2D4%2E875V12m%2D9%208%2E25h16%2E5a1%2E5%201%2E5%200%20001%2E5%2D1%2E5V5%2E25a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5H3%2E75a1%2E5%201%2E5%200%2000%2D1%2E5%201%2E5v13%2E5a1%2E5%201%2E5%200%20001%2E5%201%2E5zm12%2E621%2D9%2E44c%2D1%2E409%201%2E41%2D4%2E242%201%2E061%2D4%2E242%201%2E061s%2D%2E349%2D2%2E833%201%2E06%2D4%2E242a2%2E25%202%2E25%200%20013%2E182%203%2E182zM10%2E773%207%2E63c1%2E409%201%2E409%201%2E06%204%2E242%201%2E06%204%2E242S9%2012%2E22%207%2E592%2010%2E811a2%2E25%202%2E25%200%20113%2E182%2D3%2E182z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn GiftTopIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 3.75v16.5M2.25 12h19.5M6.375 17.25a4.875 4.875 0 004.875-4.875V12m6.375 5.25a4.875 4.875 0 01-4.875-4.875V12m-9 8.25h16.5a1.5 1.5 0 001.5-1.5V5.25a1.5 1.5 0 00-1.5-1.5H3.75a1.5 1.5 0 00-1.5 1.5v13.5a1.5 1.5 0 001.5 1.5zm12.621-9.44c-1.409 1.41-4.242 1.061-4.242 1.061s-.349-2.833 1.06-4.242a2.25 2.25 0 013.182 3.182zM10.773 7.63c1.409 1.409 1.06 4.242 1.06 4.242S9 12.22 7.592 10.811a2.25 2.25 0 113.182-3.182z"/>
</svg>
  }
}
