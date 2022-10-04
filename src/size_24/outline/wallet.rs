use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%2012a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H15a3%203%200%2011%2D6%200H5%2E25A2%2E25%202%2E25%200%20003%2012m18%200v6a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25H5%2E25A2%2E25%202%2E25%200%20013%2018v%2D6m18%200V9M3%2012V9m18%200a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H5%2E25A2%2E25%202%2E25%200%20003%209m18%200V6a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H5%2E25A2%2E25%202%2E25%200%20003%206v3%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn WalletIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a2.25 2.25 0 00-2.25-2.25H15a3 3 0 11-6 0H5.25A2.25 2.25 0 003 12m18 0v6a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 18v-6m18 0V9M3 12V9m18 0a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 9m18 0V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v3"/>
</svg>
  }
}
