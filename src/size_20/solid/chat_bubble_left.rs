use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E43%202%2E524A41%2E29%2041%2E29%200%200110%202c2%2E236%200%204%2E43%2E18%206%2E57%2E524%201%2E437%2E231%202%2E43%201%2E49%202%2E43%202%2E902v5%2E148c0%201%2E413%2D%2E993%202%2E67%2D2%2E43%202%2E902a41%2E202%2041%2E202%200%2001%2D5%2E183%2E501%2E78%2E78%200%2000%2D%2E528%2E224l%2D3%2E579%203%2E58A%2E75%2E75%200%20016%2017%2E25v%2D3%2E443a41%2E033%2041%2E033%200%2001%2D2%2E57%2D%2E33C1%2E993%2013%2E244%201%2011%2E986%201%2010%2E573V5%2E426c0%2D1%2E413%2E993%2D2%2E67%202%2E43%2D2%2E902z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.43 2.524A41.29 41.29 0 0110 2c2.236 0 4.43.18 6.57.524 1.437.231 2.43 1.49 2.43 2.902v5.148c0 1.413-.993 2.67-2.43 2.902a41.202 41.202 0 01-5.183.501.78.78 0 00-.528.224l-3.579 3.58A.75.75 0 016 17.25v-3.443a41.033 41.033 0 01-2.57-.33C1.993 13.244 1 11.986 1 10.573V5.426c0-1.413.993-2.67 2.43-2.902z" clip-rule="evenodd"/>
</svg>
  }
}