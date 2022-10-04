use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%2E22%205%2E222a%2E75%2E75%200%20011%2E06%200L7%209%2E942l3%2E768%2D3%2E769a%2E75%2E75%200%20011%2E113%2E058%2020%2E908%2020%2E908%200%20013%2E813%207%2E254l1%2E574%2D2%2E727a%2E75%2E75%200%20011%2E3%2E75l%2D2%2E475%204%2E286a%2E75%2E75%200%2001%2D1%2E025%2E275l%2D4%2E287%2D2%2E475a%2E75%2E75%200%2001%2E75%2D1%2E3l2%2E71%201%2E565a19%2E422%2019%2E422%200%2000%2D3%2E013%2D6%2E024L7%2E53%2011%2E533a%2E75%2E75%200%2001%2D1%2E06%200l%2D5%2E25%2D5%2E25a%2E75%2E75%200%20010%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowTrendingDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1.22 5.222a.75.75 0 011.06 0L7 9.942l3.768-3.769a.75.75 0 011.113.058 20.908 20.908 0 013.813 7.254l1.574-2.727a.75.75 0 011.3.75l-2.475 4.286a.75.75 0 01-1.025.275l-4.287-2.475a.75.75 0 01.75-1.3l2.71 1.565a19.422 19.422 0 00-3.013-6.024L7.53 11.533a.75.75 0 01-1.06 0l-5.25-5.25a.75.75 0 010-1.06z" clip-rule="evenodd"/>
</svg>
  }
}
