use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M1%202%2E75A%2E75%2E75%200%20011%2E75%202h16%2E5a%2E75%2E75%200%20010%201%2E5H18v8%2E75A2%2E75%202%2E75%200%200115%2E25%2015h%2D1%2E072l%2E798%203%2E06a%2E75%2E75%200%2001%2D1%2E452%2E38L13%2E41%2018H6%2E59l%2D%2E114%2E44a%2E75%2E75%200%2001%2D1%2E452%2D%2E38L5%2E823%2015H4%2E75A2%2E75%202%2E75%200%20012%2012%2E25V3%2E5h%2D%2E25A%2E75%2E75%200%20011%202%2E75zM7%2E373%2015l%2D%2E391%201%2E5h6%2E037l%2D%2E392%2D1%2E5H7%2E373zm7%2E49%2D8%2E931a%2E75%2E75%200%2001%2D%2E175%201%2E046%2019%2E326%2019%2E326%200%2000%2D3%2E398%203%2E098%2E75%2E75%200%2001%2D1%2E097%2E04L8%2E5%208%2E561l%2D2%2E22%202%2E22A%2E75%2E75%200%20115%2E22%209%2E72l2%2E75%2D2%2E75a%2E75%2E75%200%20011%2E06%200l1%2E664%201%2E663a20%2E786%2020%2E786%200%20013%2E122%2D2%2E74%2E75%2E75%200%20011%2E046%2E176z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PresentationChartLineIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h16.5a.75.75 0 010 1.5H18v8.75A2.75 2.75 0 0115.25 15h-1.072l.798 3.06a.75.75 0 01-1.452.38L13.41 18H6.59l-.114.44a.75.75 0 01-1.452-.38L5.823 15H4.75A2.75 2.75 0 012 12.25V3.5h-.25A.75.75 0 011 2.75zM7.373 15l-.391 1.5h6.037l-.392-1.5H7.373zm7.49-8.931a.75.75 0 01-.175 1.046 19.326 19.326 0 00-3.398 3.098.75.75 0 01-1.097.04L8.5 8.561l-2.22 2.22A.75.75 0 115.22 9.72l2.75-2.75a.75.75 0 011.06 0l1.664 1.663a20.786 20.786 0 013.122-2.74.75.75 0 011.046.176z" clip-rule="evenodd"/>
</svg>
  }
}
