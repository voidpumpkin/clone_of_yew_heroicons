use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%208a2%202%200%20012%2D2h%2E93a2%202%200%20001%2E664%2D%2E89l%2E812%2D1%2E22A2%202%200%20018%2E07%203h3%2E86a2%202%200%20011%2E664%2E89l%2E812%201%2E22A2%202%200%200016%2E07%206H17a2%202%200%20012%202v7a2%202%200%2001%2D2%202H3a2%202%200%2001%2D2%2D2V8zm13%2E5%203a4%2E5%204%2E5%200%2011%2D9%200%204%2E5%204%2E5%200%20019%200zM10%2014a3%203%200%20100%2D6%203%203%200%20000%206z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CameraIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1 8a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 018.07 3h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0016.07 6H17a2 2 0 012 2v7a2 2 0 01-2 2H3a2 2 0 01-2-2V8zm13.5 3a4.5 4.5 0 11-9 0 4.5 4.5 0 019 0zM10 14a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd"/>
</svg>
  }
}
