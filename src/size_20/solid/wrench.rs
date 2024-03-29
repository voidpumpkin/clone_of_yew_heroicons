use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M19%205%2E5a4%2E5%204%2E5%200%2001%2D4%2E791%204%2E49c%2D%2E873%2D%2E055%2D1%2E808%2E128%2D2%2E368%2E8l%2D6%2E024%207%2E23a2%2E724%202%2E724%200%2011%2D3%2E837%2D3%2E837L9%2E21%208%2E16c%2E672%2D%2E56%2E855%2D1%2E495%2E8%2D2%2E368a4%2E5%204%2E5%200%20015%2E873%2D4%2E575c%2E324%2E105%2E39%2E51%2E15%2E752L13%2E34%204%2E66a%2E455%2E455%200%2000%2D%2E11%2E494%203%2E01%203%2E01%200%20001%2E617%201%2E617c%2E17%2E07%2E363%2E02%2E493%2D%2E111l2%2E692%2D2%2E692c%2E241%2D%2E241%2E647%2D%2E174%2E752%2E15%2E14%2E435%2E216%2E9%2E216%201%2E382zM4%2017a1%201%200%20100%2D2%201%201%200%20000%202z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn WrenchIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M19 5.5a4.5 4.5 0 01-4.791 4.49c-.873-.055-1.808.128-2.368.8l-6.024 7.23a2.724 2.724 0 11-3.837-3.837L9.21 8.16c.672-.56.855-1.495.8-2.368a4.5 4.5 0 015.873-4.575c.324.105.39.51.15.752L13.34 4.66a.455.455 0 00-.11.494 3.01 3.01 0 001.617 1.617c.17.07.363.02.493-.111l2.692-2.692c.241-.241.647-.174.752.15.14.435.216.9.216 1.382zM4 17a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
</svg>
  }
}
