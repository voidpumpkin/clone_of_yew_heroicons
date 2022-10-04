use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%206v12m%2D3%2D2%2E818l%2E879%2E659c1%2E171%2E879%203%2E07%2E879%204%2E242%200%201%2E172%2D%2E879%201%2E172%2D2%2E303%200%2D3%2E182C13%2E536%2012%2E219%2012%2E768%2012%2012%2012c%2D%2E725%200%2D1%2E45%2D%2E22%2D2%2E003%2D%2E659%2D1%2E106%2D%2E879%2D1%2E106%2D2%2E303%200%2D3%2E182s2%2E9%2D%2E879%204%2E006%200l%2E415%2E33M21%2012a9%209%200%2011%2D18%200%209%209%200%200118%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CurrencyDollarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
</svg>
  }
}
