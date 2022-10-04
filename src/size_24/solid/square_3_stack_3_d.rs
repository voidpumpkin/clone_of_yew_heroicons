use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M11%2E644%201%2E59a%2E75%2E75%200%2001%2E712%200l9%2E75%205%2E25a%2E75%2E75%200%20010%201%2E32l%2D9%2E75%205%2E25a%2E75%2E75%200%2001%2D%2E712%200l%2D9%2E75%2D5%2E25a%2E75%2E75%200%20010%2D1%2E32l9%2E75%2D5%2E25z%22%2F%3E%20%3Cpath%20d%3D%22M3%2E265%2010%2E602l7%2E668%204%2E129a2%2E25%202%2E25%200%20002%2E134%200l7%2E668%2D4%2E13%201%2E37%2E739a%2E75%2E75%200%20010%201%2E32l%2D9%2E75%205%2E25a%2E75%2E75%200%2001%2D%2E71%200l%2D9%2E75%2D5%2E25a%2E75%2E75%200%20010%2D1%2E32l1%2E37%2D%2E738z%22%2F%3E%20%3Cpath%20d%3D%22M10%2E933%2019%2E231l%2D7%2E668%2D4%2E13%2D1%2E37%2E739a%2E75%2E75%200%20000%201%2E32l9%2E75%205%2E25c%2E221%2E12%2E489%2E12%2E71%200l9%2E75%2D5%2E25a%2E75%2E75%200%20000%2D1%2E32l%2D1%2E37%2D%2E738%2D7%2E668%204%2E13a2%2E25%202%2E25%200%2001%2D2%2E134%2D%2E001z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Square3Stack3DIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M11.644 1.59a.75.75 0 01.712 0l9.75 5.25a.75.75 0 010 1.32l-9.75 5.25a.75.75 0 01-.712 0l-9.75-5.25a.75.75 0 010-1.32l9.75-5.25z"/>
  <path d="M3.265 10.602l7.668 4.129a2.25 2.25 0 002.134 0l7.668-4.13 1.37.739a.75.75 0 010 1.32l-9.75 5.25a.75.75 0 01-.71 0l-9.75-5.25a.75.75 0 010-1.32l1.37-.738z"/>
  <path d="M10.933 19.231l-7.668-4.13-1.37.739a.75.75 0 000 1.32l9.75 5.25c.221.12.489.12.71 0l9.75-5.25a.75.75 0 000-1.32l-1.37-.738-7.668 4.13a2.25 2.25 0 01-2.134-.001z"/>
</svg>
  }
}