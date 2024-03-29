use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%202c%2D2%2E236%200%2D4%2E43%2E18%2D6%2E57%2E524C1%2E993%202%2E755%201%204%2E014%201%205%2E426v5%2E148c0%201%2E413%2E993%202%2E67%202%2E43%202%2E902%2E848%2E137%201%2E705%2E248%202%2E57%2E331v3%2E443a%2E75%2E75%200%20001%2E28%2E53l3%2E58%2D3%2E579a%2E78%2E78%200%2001%2E527%2D%2E224%2041%2E202%2041%2E202%200%20005%2E183%2D%2E5c1%2E437%2D%2E232%202%2E43%2D1%2E49%202%2E43%2D2%2E903V5%2E426c0%2D1%2E413%2D%2E993%2D2%2E67%2D2%2E43%2D2%2E902A41%2E289%2041%2E289%200%200010%202zm0%207a1%201%200%20100%2D2%201%201%200%20000%202zM8%208a1%201%200%2011%2D2%200%201%201%200%20012%200zm5%201a1%201%200%20100%2D2%201%201%200%20000%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ChatBubbleLeftEllipsisIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 2c-2.236 0-4.43.18-6.57.524C1.993 2.755 1 4.014 1 5.426v5.148c0 1.413.993 2.67 2.43 2.902.848.137 1.705.248 2.57.331v3.443a.75.75 0 001.28.53l3.58-3.579a.78.78 0 01.527-.224 41.202 41.202 0 005.183-.5c1.437-.232 2.43-1.49 2.43-2.903V5.426c0-1.413-.993-2.67-2.43-2.902A41.289 41.289 0 0010 2zm0 7a1 1 0 100-2 1 1 0 000 2zM8 8a1 1 0 11-2 0 1 1 0 012 0zm5 1a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
</svg>
  }
}
