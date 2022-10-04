use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%2012%2E5a2%2E5%202%2E5%200%20100%2D5%202%2E5%202%2E5%200%20000%205z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M%2E664%2010%2E59a1%2E651%201%2E651%200%20010%2D1%2E186A10%2E004%2010%2E004%200%200110%203c4%2E257%200%207%2E893%202%2E66%209%2E336%206%2E41%2E147%2E381%2E146%2E804%200%201%2E186A10%2E004%2010%2E004%200%200110%2017c%2D4%2E257%200%2D7%2E893%2D2%2E66%2D9%2E336%2D6%2E41zM14%2010a4%204%200%2011%2D8%200%204%204%200%20018%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EyeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10 12.5a2.5 2.5 0 100-5 2.5 2.5 0 000 5z"/>
  <path fill-rule="evenodd" d="M.664 10.59a1.651 1.651 0 010-1.186A10.004 10.004 0 0110 3c4.257 0 7.893 2.66 9.336 6.41.147.381.146.804 0 1.186A10.004 10.004 0 0110 17c-4.257 0-7.893-2.66-9.336-6.41zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"/>
</svg>
  }
}