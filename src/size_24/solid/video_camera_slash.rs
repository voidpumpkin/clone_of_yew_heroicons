use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M3%2E53%202%2E47a%2E75%2E75%200%2000%2D1%2E06%201%2E06l18%2018a%2E75%2E75%200%20101%2E06%2D1%2E06l%2D18%2D18zM22%2E5%2017%2E69c0%20%2E471%2D%2E202%2E86%2D%2E504%201%2E124l%2D4%2E746%2D4%2E746V7%2E939l2%2E69%2D2%2E689c%2E944%2D%2E945%202%2E56%2D%2E276%202%2E56%201%2E06v11%2E38zM15%2E75%207%2E5v5%2E068L7%2E682%204%2E5h5%2E068a3%203%200%20013%203zM1%2E5%207%2E5c0%2D%2E782%2E3%2D1%2E494%2E79%2D2%2E028l12%2E846%2012%2E846A2%2E995%202%2E995%200%200112%2E75%2019%2E5H4%2E5a3%203%200%2001%2D3%2D3v%2D9z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn VideoCameraSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M3.53 2.47a.75.75 0 00-1.06 1.06l18 18a.75.75 0 101.06-1.06l-18-18zM22.5 17.69c0 .471-.202.86-.504 1.124l-4.746-4.746V7.939l2.69-2.689c.944-.945 2.56-.276 2.56 1.06v11.38zM15.75 7.5v5.068L7.682 4.5h5.068a3 3 0 013 3zM1.5 7.5c0-.782.3-1.494.79-2.028l12.846 12.846A2.995 2.995 0 0112.75 19.5H4.5a3 3 0 01-3-3v-9z"/>
</svg>
  }
}
