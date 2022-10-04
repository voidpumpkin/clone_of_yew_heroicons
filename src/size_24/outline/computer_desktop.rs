use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%2017%2E25v1%2E007a3%203%200%2001%2D%2E879%202%2E122L7%2E5%2021h9l%2D%2E621%2D%2E621A3%203%200%200115%2018%2E257V17%2E25m6%2D12V15a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25H5%2E25A2%2E25%202%2E25%200%20013%2015V5%2E25m18%200A2%2E25%202%2E25%200%200018%2E75%203H5%2E25A2%2E25%202%2E25%200%20003%205%2E25m18%200V12a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25H5%2E25A2%2E25%202%2E25%200%20013%2012V5%2E25%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ComputerDesktopIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25m18 0A2.25 2.25 0 0018.75 3H5.25A2.25 2.25 0 003 5.25m18 0V12a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 12V5.25"/>
</svg>
  }
}