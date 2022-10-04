use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M19%2E5%2021a3%203%200%20003%2D3v%2D4%2E5a3%203%200%2000%2D3%2D3h%2D15a3%203%200%2000%2D3%203V18a3%203%200%20003%203h15zM1%2E5%2010%2E146V6a3%203%200%20013%2D3h5%2E379a2%2E25%202%2E25%200%20011%2E59%2E659l2%2E122%202%2E121c%2E14%2E141%2E331%2E22%2E53%2E22H19%2E5a3%203%200%20013%203v1%2E146A4%2E483%204%2E483%200%200019%2E5%209h%2D15a4%2E483%204%2E483%200%2000%2D3%201%2E146z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FolderIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M19.5 21a3 3 0 003-3v-4.5a3 3 0 00-3-3h-15a3 3 0 00-3 3V18a3 3 0 003 3h15zM1.5 10.146V6a3 3 0 013-3h5.379a2.25 2.25 0 011.59.659l2.122 2.121c.14.141.331.22.53.22H19.5a3 3 0 013 3v1.146A4.483 4.483 0 0019.5 9h-15a4.483 4.483 0 00-3 1.146z"/>
</svg>
  }
}