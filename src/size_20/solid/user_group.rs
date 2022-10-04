use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%209a3%203%200%20100%2D6%203%203%200%20000%206zM6%208a2%202%200%2011%2D4%200%202%202%200%20014%200zM1%2E49%2015%2E326a%2E78%2E78%200%2001%2D%2E358%2D%2E442%203%203%200%20014%2E308%2D3%2E516%206%2E484%206%2E484%200%2000%2D1%2E905%203%2E959c%2D%2E023%2E222%2D%2E014%2E442%2E025%2E654a4%2E97%204%2E97%200%2001%2D2%2E07%2D%2E655zM16%2E44%2015%2E98a4%2E97%204%2E97%200%20002%2E07%2D%2E654%2E78%2E78%200%2000%2E357%2D%2E442%203%203%200%2000%2D4%2E308%2D3%2E517%206%2E484%206%2E484%200%20011%2E907%203%2E96%202%2E32%202%2E32%200%2001%2D%2E026%2E654zM18%208a2%202%200%2011%2D4%200%202%202%200%20014%200zM5%2E304%2016%2E19a%2E844%2E844%200%2001%2D%2E277%2D%2E71%205%205%200%20019%2E947%200%20%2E843%2E843%200%2001%2D%2E277%2E71A6%2E975%206%2E975%200%200110%2018a6%2E974%206%2E974%200%2001%2D4%2E696%2D1%2E81z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn UserGroupIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10 9a3 3 0 100-6 3 3 0 000 6zM6 8a2 2 0 11-4 0 2 2 0 014 0zM1.49 15.326a.78.78 0 01-.358-.442 3 3 0 014.308-3.516 6.484 6.484 0 00-1.905 3.959c-.023.222-.014.442.025.654a4.97 4.97 0 01-2.07-.655zM16.44 15.98a4.97 4.97 0 002.07-.654.78.78 0 00.357-.442 3 3 0 00-4.308-3.517 6.484 6.484 0 011.907 3.96 2.32 2.32 0 01-.026.654zM18 8a2 2 0 11-4 0 2 2 0 014 0zM5.304 16.19a.844.844 0 01-.277-.71 5 5 0 019.947 0 .843.843 0 01-.277.71A6.975 6.975 0 0110 18a6.974 6.974 0 01-4.696-1.81z"/>
</svg>
  }
}
