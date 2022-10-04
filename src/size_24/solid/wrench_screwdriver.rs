use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%206%2E75a5%2E25%205%2E25%200%20016%2E775%2D5%2E025%2E75%2E75%200%2001%2E313%201%2E248l%2D3%2E32%203%2E319c%2E063%2E475%2E276%2E934%2E641%201%2E299%2E365%2E365%2E824%2E578%201%2E3%2E64l3%2E318%2D3%2E319a%2E75%2E75%200%20011%2E248%2E313%205%2E25%205%2E25%200%2001%2D5%2E472%206%2E756c%2D1%2E018%2D%2E086%2D1%2E87%2E1%2D2%2E309%2E634L7%2E344%2021%2E3A3%2E298%203%2E298%200%20112%2E7%2016%2E657l8%2E684%2D7%2E151c%2E533%2D%2E44%2E72%2D1%2E291%2E634%2D2%2E309A5%2E342%205%2E342%200%200112%206%2E75zM4%2E117%2019%2E125a%2E75%2E75%200%2001%2E75%2D%2E75h%2E008a%2E75%2E75%200%2001%2E75%2E75v%2E008a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E008a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D%2E008z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M10%2E076%208%2E64l%2D2%2E201%2D2%2E2V4%2E874a%2E75%2E75%200%2000%2D%2E364%2D%2E643l%2D3%2E75%2D2%2E25a%2E75%2E75%200%2000%2D%2E916%2E113l%2D%2E75%2E75a%2E75%2E75%200%2000%2D%2E113%2E916l2%2E25%203%2E75a%2E75%2E75%200%2000%2E643%2E364h1%2E564l2%2E062%202%2E062%201%2E575%2D1%2E297z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%2E556%2017%2E329l4%2E183%204%2E182a3%2E375%203%2E375%200%20004%2E773%2D4%2E773l%2D3%2E306%2D3%2E305a6%2E803%206%2E803%200%2001%2D1%2E53%2E043c%2D%2E394%2D%2E034%2D%2E682%2D%2E006%2D%2E867%2E042a%2E589%2E589%200%2000%2D%2E167%2E063l%2D3%2E086%203%2E748zm3%2E414%2D1%2E36a%2E75%2E75%200%20011%2E06%200l1%2E875%201%2E876a%2E75%2E75%200%2011%2D1%2E06%201%2E06L15%2E97%2017%2E03a%2E75%2E75%200%20010%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn WrenchScrewdriverIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 6.75a5.25 5.25 0 016.775-5.025.75.75 0 01.313 1.248l-3.32 3.319c.063.475.276.934.641 1.299.365.365.824.578 1.3.64l3.318-3.319a.75.75 0 011.248.313 5.25 5.25 0 01-5.472 6.756c-1.018-.086-1.87.1-2.309.634L7.344 21.3A3.298 3.298 0 112.7 16.657l8.684-7.151c.533-.44.72-1.291.634-2.309A5.342 5.342 0 0112 6.75zM4.117 19.125a.75.75 0 01.75-.75h.008a.75.75 0 01.75.75v.008a.75.75 0 01-.75.75h-.008a.75.75 0 01-.75-.75v-.008z" clip-rule="evenodd"/>
  <path d="M10.076 8.64l-2.201-2.2V4.874a.75.75 0 00-.364-.643l-3.75-2.25a.75.75 0 00-.916.113l-.75.75a.75.75 0 00-.113.916l2.25 3.75a.75.75 0 00.643.364h1.564l2.062 2.062 1.575-1.297z"/>
  <path fill-rule="evenodd" d="M12.556 17.329l4.183 4.182a3.375 3.375 0 004.773-4.773l-3.306-3.305a6.803 6.803 0 01-1.53.043c-.394-.034-.682-.006-.867.042a.589.589 0 00-.167.063l-3.086 3.748zm3.414-1.36a.75.75 0 011.06 0l1.875 1.876a.75.75 0 11-1.06 1.06L15.97 17.03a.75.75 0 010-1.06z" clip-rule="evenodd"/>
</svg>
  }
}