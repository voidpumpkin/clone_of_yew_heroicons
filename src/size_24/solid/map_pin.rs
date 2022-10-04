use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M11%2E54%2022%2E351l%2E07%2E04%2E028%2E016a%2E76%2E76%200%2000%2E723%200l%2E028%2D%2E015%2E071%2D%2E041a16%2E975%2016%2E975%200%20001%2E144%2D%2E742%2019%2E58%2019%2E58%200%20002%2E683%2D2%2E282c1%2E944%2D1%2E99%203%2E963%2D4%2E98%203%2E963%2D8%2E827a8%2E25%208%2E25%200%2000%2D16%2E5%200c0%203%2E846%202%2E02%206%2E837%203%2E963%208%2E827a19%2E58%2019%2E58%200%20002%2E682%202%2E282%2016%2E975%2016%2E975%200%20001%2E145%2E742zM12%2013%2E5a3%203%200%20100%2D6%203%203%200%20000%206z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MapPinIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M11.54 22.351l.07.04.028.016a.76.76 0 00.723 0l.028-.015.071-.041a16.975 16.975 0 001.144-.742 19.58 19.58 0 002.683-2.282c1.944-1.99 3.963-4.98 3.963-8.827a8.25 8.25 0 00-16.5 0c0 3.846 2.02 6.837 3.963 8.827a19.58 19.58 0 002.682 2.282 16.975 16.975 0 001.145.742zM12 13.5a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd"/>
</svg>
  }
}