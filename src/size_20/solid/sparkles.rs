use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M15%2E98%201%2E804a1%201%200%2000%2D1%2E96%200l%2D%2E24%201%2E192a1%201%200%2001%2D%2E784%2E785l%2D1%2E192%2E238a1%201%200%20000%201%2E962l1%2E192%2E238a1%201%200%2001%2E785%2E785l%2E238%201%2E192a1%201%200%20001%2E962%200l%2E238%2D1%2E192a1%201%200%2001%2E785%2D%2E785l1%2E192%2D%2E238a1%201%200%20000%2D1%2E962l%2D1%2E192%2D%2E238a1%201%200%2001%2D%2E785%2D%2E785l%2D%2E238%2D1%2E192zM6%2E949%205%2E684a1%201%200%2000%2D1%2E898%200l%2D%2E683%202%2E051a1%201%200%2001%2D%2E633%2E633l%2D2%2E051%2E683a1%201%200%20000%201%2E898l2%2E051%2E684a1%201%200%2001%2E633%2E632l%2E683%202%2E051a1%201%200%20001%2E898%200l%2E683%2D2%2E051a1%201%200%2001%2E633%2D%2E633l2%2E051%2D%2E683a1%201%200%20000%2D1%2E898l%2D2%2E051%2D%2E683a1%201%200%2001%2D%2E633%2D%2E633L6%2E95%205%2E684zM13%2E949%2013%2E684a1%201%200%2000%2D1%2E898%200l%2D%2E184%2E551a1%201%200%2001%2D%2E632%2E633l%2D%2E551%2E183a1%201%200%20000%201%2E898l%2E551%2E183a1%201%200%2001%2E633%2E633l%2E183%2E551a1%201%200%20001%2E898%200l%2E184%2D%2E551a1%201%200%2001%2E632%2D%2E633l%2E551%2D%2E183a1%201%200%20000%2D1%2E898l%2D%2E551%2D%2E184a1%201%200%2001%2D%2E633%2D%2E632l%2D%2E183%2D%2E551z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SparklesIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M15.98 1.804a1 1 0 00-1.96 0l-.24 1.192a1 1 0 01-.784.785l-1.192.238a1 1 0 000 1.962l1.192.238a1 1 0 01.785.785l.238 1.192a1 1 0 001.962 0l.238-1.192a1 1 0 01.785-.785l1.192-.238a1 1 0 000-1.962l-1.192-.238a1 1 0 01-.785-.785l-.238-1.192zM6.949 5.684a1 1 0 00-1.898 0l-.683 2.051a1 1 0 01-.633.633l-2.051.683a1 1 0 000 1.898l2.051.684a1 1 0 01.633.632l.683 2.051a1 1 0 001.898 0l.683-2.051a1 1 0 01.633-.633l2.051-.683a1 1 0 000-1.898l-2.051-.683a1 1 0 01-.633-.633L6.95 5.684zM13.949 13.684a1 1 0 00-1.898 0l-.184.551a1 1 0 01-.632.633l-.551.183a1 1 0 000 1.898l.551.183a1 1 0 01.633.633l.183.551a1 1 0 001.898 0l.184-.551a1 1 0 01.632-.633l.551-.183a1 1 0 000-1.898l-.551-.184a1 1 0 01-.633-.632l-.183-.551z"/>
</svg>
  }
}