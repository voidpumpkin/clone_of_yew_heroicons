use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%2E315%207%2E584C12%2E195%203%2E883%2016%2E695%201%2E5%2021%2E75%201%2E5a%2E75%2E75%200%2001%2E75%2E75c0%205%2E056%2D2%2E383%209%2E555%2D6%2E084%2012%2E436A6%2E75%206%2E75%200%20019%2E75%2022%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D4%2E131A15%2E838%2015%2E838%200%20016%2E382%2015H2%2E25a%2E75%2E75%200%2001%2D%2E75%2D%2E75%206%2E75%206%2E75%200%20017%2E815%2D6%2E666zM15%206%2E75a2%2E25%202%2E25%200%20100%204%2E5%202%2E25%202%2E25%200%20000%2D4%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M5%2E26%2017%2E242a%2E75%2E75%200%2010%2D%2E897%2D1%2E203%205%2E243%205%2E243%200%2000%2D2%2E05%205%2E022%2E75%2E75%200%2000%2E625%2E627%205%2E243%205%2E243%200%20005%2E022%2D2%2E051%2E75%2E75%200%2010%2D1%2E202%2D%2E897%203%2E744%203%2E744%200%2001%2D3%2E008%201%2E51c0%2D1%2E23%2E592%2D2%2E323%201%2E51%2D3%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RocketLaunchIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M9.315 7.584C12.195 3.883 16.695 1.5 21.75 1.5a.75.75 0 01.75.75c0 5.056-2.383 9.555-6.084 12.436A6.75 6.75 0 019.75 22.5a.75.75 0 01-.75-.75v-4.131A15.838 15.838 0 016.382 15H2.25a.75.75 0 01-.75-.75 6.75 6.75 0 017.815-6.666zM15 6.75a2.25 2.25 0 100 4.5 2.25 2.25 0 000-4.5z" clip-rule="evenodd"/>
  <path d="M5.26 17.242a.75.75 0 10-.897-1.203 5.243 5.243 0 00-2.05 5.022.75.75 0 00.625.627 5.243 5.243 0 005.022-2.051.75.75 0 10-1.202-.897 3.744 3.744 0 01-3.008 1.51c0-1.23.592-2.323 1.51-3.008z"/>
</svg>
  }
}