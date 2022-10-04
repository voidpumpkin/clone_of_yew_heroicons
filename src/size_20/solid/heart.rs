use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M9%2E653%2016%2E915l%2D%2E005%2D%2E003%2D%2E019%2D%2E01a20%2E759%2020%2E759%200%2001%2D1%2E162%2D%2E682%2022%2E045%2022%2E045%200%2001%2D2%2E582%2D1%2E9C4%2E045%2012%2E733%202%2010%2E352%202%207%2E5a4%2E5%204%2E5%200%20018%2D2%2E828A4%2E5%204%2E5%200%200118%207%2E5c0%202%2E852%2D2%2E044%205%2E233%2D3%2E885%206%2E82a22%2E049%2022%2E049%200%2001%2D3%2E744%202%2E582l%2D%2E019%2E01%2D%2E005%2E003h%2D%2E002a%2E739%2E739%200%2001%2D%2E69%2E001l%2D%2E002%2D%2E001z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HeartIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M9.653 16.915l-.005-.003-.019-.01a20.759 20.759 0 01-1.162-.682 22.045 22.045 0 01-2.582-1.9C4.045 12.733 2 10.352 2 7.5a4.5 4.5 0 018-2.828A4.5 4.5 0 0118 7.5c0 2.852-2.044 5.233-3.885 6.82a22.049 22.049 0 01-3.744 2.582l-.019.01-.005.003h-.002a.739.739 0 01-.69.001l-.002-.001z"/>
</svg>
  }
}
