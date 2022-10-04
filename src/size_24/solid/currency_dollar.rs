use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%2E464%208%2E746c%2E227%2D%2E18%2E497%2D%2E311%2E786%2D%2E394v2%2E795a2%2E252%202%2E252%200%2001%2D%2E786%2D%2E393c%2D%2E394%2D%2E313%2D%2E546%2D%2E681%2D%2E546%2D1%2E004%200%2D%2E323%2E152%2D%2E691%2E546%2D1%2E004zM12%2E75%2015%2E662v%2D2%2E824c%2E347%2E085%2E664%2E228%2E921%2E421%2E427%2E32%2E579%2E686%2E579%2E991%200%20%2E305%2D%2E152%2E671%2D%2E579%2E991a2%2E534%202%2E534%200%2001%2D%2E921%2E42z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%202%2E25c%2D5%2E385%200%2D9%2E75%204%2E365%2D9%2E75%209%2E75s4%2E365%209%2E75%209%2E75%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75S17%2E385%202%2E25%2012%202%2E25zM12%2E75%206a%2E75%2E75%200%2000%2D1%2E5%200v%2E816a3%2E836%203%2E836%200%2000%2D1%2E72%2E756c%2D%2E712%2E566%2D1%2E112%201%2E35%2D1%2E112%202%2E178%200%20%2E829%2E4%201%2E612%201%2E113%202%2E178%2E502%2E4%201%2E102%2E647%201%2E719%2E756v2%2E978a2%2E536%202%2E536%200%2001%2D%2E921%2D%2E421l%2D%2E879%2D%2E66a%2E75%2E75%200%2000%2D%2E9%201%2E2l%2E879%2E66c%2E533%2E4%201%2E169%2E645%201%2E821%2E75V18a%2E75%2E75%200%20001%2E5%200v%2D%2E81a4%2E124%204%2E124%200%20001%2E821%2D%2E749c%2E745%2D%2E559%201%2E179%2D1%2E344%201%2E179%2D2%2E191%200%2D%2E847%2D%2E434%2D1%2E632%2D1%2E179%2D2%2E191a4%2E122%204%2E122%200%2000%2D1%2E821%2D%2E75V8%2E354c%2E29%2E082%2E559%2E213%2E786%2E393l%2E415%2E33a%2E75%2E75%200%2000%2E933%2D1%2E175l%2D%2E415%2D%2E33a3%2E836%203%2E836%200%2000%2D1%2E719%2D%2E755V6z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CurrencyDollarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M10.464 8.746c.227-.18.497-.311.786-.394v2.795a2.252 2.252 0 01-.786-.393c-.394-.313-.546-.681-.546-1.004 0-.323.152-.691.546-1.004zM12.75 15.662v-2.824c.347.085.664.228.921.421.427.32.579.686.579.991 0 .305-.152.671-.579.991a2.534 2.534 0 01-.921.42z"/>
  <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zM12.75 6a.75.75 0 00-1.5 0v.816a3.836 3.836 0 00-1.72.756c-.712.566-1.112 1.35-1.112 2.178 0 .829.4 1.612 1.113 2.178.502.4 1.102.647 1.719.756v2.978a2.536 2.536 0 01-.921-.421l-.879-.66a.75.75 0 00-.9 1.2l.879.66c.533.4 1.169.645 1.821.75V18a.75.75 0 001.5 0v-.81a4.124 4.124 0 001.821-.749c.745-.559 1.179-1.344 1.179-2.191 0-.847-.434-1.632-1.179-2.191a4.122 4.122 0 00-1.821-.75V8.354c.29.082.559.213.786.393l.415.33a.75.75 0 00.933-1.175l-.415-.33a3.836 3.836 0 00-1.719-.755V6z" clip-rule="evenodd"/>
</svg>
  }
}