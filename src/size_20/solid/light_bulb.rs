use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%201a6%206%200%2000%2D3%2E815%2010%2E631C7%2E237%2012%2E5%208%2013%2E443%208%2014%2E456v%2E644a%2E75%2E75%200%2000%2E572%2E729%206%2E016%206%2E016%200%20002%2E856%200A%2E75%2E75%200%200012%2015%2E1v%2D%2E644c0%2D1%2E013%2E762%2D1%2E957%201%2E815%2D2%2E825A6%206%200%200010%201zM8%2E863%2017%2E414a%2E75%2E75%200%2000%2D%2E226%201%2E483%209%2E066%209%2E066%200%20002%2E726%200%20%2E75%2E75%200%2000%2D%2E226%2D1%2E483%207%2E553%207%2E553%200%2001%2D2%2E274%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LightBulbIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10 1a6 6 0 00-3.815 10.631C7.237 12.5 8 13.443 8 14.456v.644a.75.75 0 00.572.729 6.016 6.016 0 002.856 0A.75.75 0 0012 15.1v-.644c0-1.013.762-1.957 1.815-2.825A6 6 0 0010 1zM8.863 17.414a.75.75 0 00-.226 1.483 9.066 9.066 0 002.726 0 .75.75 0 00-.226-1.483 7.553 7.553 0 01-2.274 0z"/>
</svg>
  }
}
