use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%2021a9%2E004%209%2E004%200%20008%2E716%2D6%2E747M12%2021a9%2E004%209%2E004%200%2001%2D8%2E716%2D6%2E747M12%2021c2%2E485%200%204%2E5%2D4%2E03%204%2E5%2D9S14%2E485%203%2012%203m0%2018c%2D2%2E485%200%2D4%2E5%2D4%2E03%2D4%2E5%2D9S9%2E515%203%2012%203m0%200a8%2E997%208%2E997%200%20017%2E843%204%2E582M12%203a8%2E997%208%2E997%200%2000%2D7%2E843%204%2E582m15%2E686%200A11%2E953%2011%2E953%200%200112%2010%2E5c%2D2%2E998%200%2D5%2E74%2D1%2E1%2D7%2E843%2D2%2E918m15%2E686%200A8%2E959%208%2E959%200%200121%2012c0%20%2E778%2D%2E099%201%2E533%2D%2E284%202%2E253m0%200A17%2E919%2017%2E919%200%200112%2016%2E5c%2D3%2E162%200%2D6%2E133%2D%2E815%2D8%2E716%2D2%2E247m0%200A9%2E015%209%2E015%200%20013%2012c0%2D1%2E605%2E42%2D3%2E113%201%2E157%2D4%2E418%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn GlobeAltIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0112 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 013 12c0-1.605.42-3.113 1.157-4.418"/>
</svg>
  }
}
