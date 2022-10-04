use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E5%206%2E375a4%2E125%204%2E125%200%20118%2E25%200%204%2E125%204%2E125%200%2001%2D8%2E25%200zM14%2E25%208%2E625a3%2E375%203%2E375%200%20116%2E75%200%203%2E375%203%2E375%200%2001%2D6%2E75%200zM1%2E5%2019%2E125a7%2E125%207%2E125%200%200114%2E25%200v%2E003l%2D%2E001%2E119a%2E75%2E75%200%2001%2D%2E363%2E63%2013%2E067%2013%2E067%200%2001%2D6%2E761%201%2E873c%2D2%2E472%200%2D4%2E786%2D%2E684%2D6%2E76%2D1%2E873a%2E75%2E75%200%2001%2D%2E364%2D%2E63l%2D%2E001%2D%2E122zM17%2E25%2019%2E128l%2D%2E001%2E144a2%2E25%202%2E25%200%2001%2D%2E233%2E96%2010%2E088%2010%2E088%200%20005%2E06%2D1%2E01%2E75%2E75%200%2000%2E42%2D%2E643%204%2E875%204%2E875%200%2000%2D6%2E957%2D4%2E611%208%2E586%208%2E586%200%20011%2E71%205%2E157v%2E003z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UsersIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M4.5 6.375a4.125 4.125 0 118.25 0 4.125 4.125 0 01-8.25 0zM14.25 8.625a3.375 3.375 0 116.75 0 3.375 3.375 0 01-6.75 0zM1.5 19.125a7.125 7.125 0 0114.25 0v.003l-.001.119a.75.75 0 01-.363.63 13.067 13.067 0 01-6.761 1.873c-2.472 0-4.786-.684-6.76-1.873a.75.75 0 01-.364-.63l-.001-.122zM17.25 19.128l-.001.144a2.25 2.25 0 01-.233.96 10.088 10.088 0 005.06-1.01.75.75 0 00.42-.643 4.875 4.875 0 00-6.957-4.611 8.586 8.586 0 011.71 5.157v.003z"/>
</svg>
  }
}