use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M18%2018%2E72a9%2E094%209%2E094%200%20003%2E741%2D%2E479%203%203%200%2000%2D4%2E682%2D2%2E72m%2E94%203%2E198l%2E001%2E031c0%20%2E225%2D%2E012%2E447%2D%2E037%2E666A11%2E944%2011%2E944%200%200112%2021c%2D2%2E17%200%2D4%2E207%2D%2E576%2D5%2E963%2D1%2E584A6%2E062%206%2E062%200%20016%2018%2E719m12%200a5%2E971%205%2E971%200%2000%2D%2E941%2D3%2E197m0%200A5%2E995%205%2E995%200%200012%2012%2E75a5%2E995%205%2E995%200%2000%2D5%2E058%202%2E772m0%200a3%203%200%2000%2D4%2E681%202%2E72%208%2E986%208%2E986%200%20003%2E74%2E477m%2E94%2D3%2E197a5%2E971%205%2E971%200%2000%2D%2E94%203%2E197M15%206%2E75a3%203%200%2011%2D6%200%203%203%200%20016%200zm6%203a2%2E25%202%2E25%200%2011%2D4%2E5%200%202%2E25%202%2E25%200%20014%2E5%200zm%2D13%2E5%200a2%2E25%202%2E25%200%2011%2D4%2E5%200%202%2E25%202%2E25%200%20014%2E5%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserGroupIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M18 18.72a9.094 9.094 0 003.741-.479 3 3 0 00-4.682-2.72m.94 3.198l.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0112 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 016 18.719m12 0a5.971 5.971 0 00-.941-3.197m0 0A5.995 5.995 0 0012 12.75a5.995 5.995 0 00-5.058 2.772m0 0a3 3 0 00-4.681 2.72 8.986 8.986 0 003.74.477m.94-3.197a5.971 5.971 0 00-.94 3.197M15 6.75a3 3 0 11-6 0 3 3 0 016 0zm6 3a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0zm-13.5 0a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z"/>
</svg>
  }
}
