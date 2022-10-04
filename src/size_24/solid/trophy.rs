use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E166%202%2E621v%2E858c%2D1%2E035%2E148%2D2%2E059%2E33%2D3%2E071%2E543a%2E75%2E75%200%2000%2D%2E584%2E859%206%2E753%206%2E753%200%20006%2E138%205%2E6%206%2E73%206%2E73%200%20002%2E743%201%2E346A6%2E707%206%2E707%200%20019%2E279%2015H8%2E54c%2D1%2E036%200%2D1%2E875%2E84%2D1%2E875%201%2E875V19%2E5h%2D%2E75a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25c0%20%2E414%2E336%2E75%2E75%2E75h15a%2E75%2E75%200%2000%2E75%2D%2E75%202%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25h%2D%2E75v%2D2%2E625c0%2D1%2E036%2D%2E84%2D1%2E875%2D1%2E875%2D1%2E875h%2D%2E739a6%2E706%206%2E706%200%2001%2D1%2E112%2D3%2E173%206%2E73%206%2E73%200%20002%2E743%2D1%2E347%206%2E753%206%2E753%200%20006%2E139%2D5%2E6%2E75%2E75%200%2000%2D%2E585%2D%2E858%2047%2E077%2047%2E077%200%2000%2D3%2E07%2D%2E543V2%2E62a%2E75%2E75%200%2000%2D%2E658%2D%2E744%2049%2E22%2049%2E22%200%2000%2D6%2E093%2D%2E377c%2D2%2E063%200%2D4%2E096%2E128%2D6%2E093%2E377a%2E75%2E75%200%2000%2D%2E657%2E744zm0%202%2E629c0%201%2E196%2E312%202%2E32%2E857%203%2E294A5%2E266%205%2E266%200%20013%2E16%205%2E337a45%2E6%2045%2E6%200%20012%2E006%2D%2E343v%2E256zm13%2E5%200v%2D%2E256c%2E674%2E1%201%2E343%2E214%202%2E006%2E343a5%2E265%205%2E265%200%2001%2D2%2E863%203%2E207%206%2E72%206%2E72%200%2000%2E857%2D3%2E294z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TrophyIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.166 2.621v.858c-1.035.148-2.059.33-3.071.543a.75.75 0 00-.584.859 6.753 6.753 0 006.138 5.6 6.73 6.73 0 002.743 1.346A6.707 6.707 0 019.279 15H8.54c-1.036 0-1.875.84-1.875 1.875V19.5h-.75a2.25 2.25 0 00-2.25 2.25c0 .414.336.75.75.75h15a.75.75 0 00.75-.75 2.25 2.25 0 00-2.25-2.25h-.75v-2.625c0-1.036-.84-1.875-1.875-1.875h-.739a6.706 6.706 0 01-1.112-3.173 6.73 6.73 0 002.743-1.347 6.753 6.753 0 006.139-5.6.75.75 0 00-.585-.858 47.077 47.077 0 00-3.07-.543V2.62a.75.75 0 00-.658-.744 49.22 49.22 0 00-6.093-.377c-2.063 0-4.096.128-6.093.377a.75.75 0 00-.657.744zm0 2.629c0 1.196.312 2.32.857 3.294A5.266 5.266 0 013.16 5.337a45.6 45.6 0 012.006-.343v.256zm13.5 0v-.256c.674.1 1.343.214 2.006.343a5.265 5.265 0 01-2.863 3.207 6.72 6.72 0 00.857-3.294z" clip-rule="evenodd"/>
</svg>
  }
}
