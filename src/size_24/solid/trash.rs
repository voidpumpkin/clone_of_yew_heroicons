use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M16%2E5%204%2E478v%2E227a48%2E816%2048%2E816%200%20013%2E878%2E512%2E75%2E75%200%2011%2D%2E256%201%2E478l%2D%2E209%2D%2E035%2D1%2E005%2013%2E07a3%203%200%2001%2D2%2E991%202%2E77H8%2E084a3%203%200%2001%2D2%2E991%2D2%2E77L4%2E087%206%2E66l%2D%2E209%2E035a%2E75%2E75%200%2001%2D%2E256%2D1%2E478A48%2E567%2048%2E567%200%20017%2E5%204%2E705v%2D%2E227c0%2D1%2E564%201%2E213%2D2%2E9%202%2E816%2D2%2E951a52%2E662%2052%2E662%200%20013%2E369%200c1%2E603%2E051%202%2E815%201%2E387%202%2E815%202%2E951zm%2D6%2E136%2D1%2E452a51%2E196%2051%2E196%200%20013%2E273%200C14%2E39%203%2E05%2015%203%2E684%2015%204%2E478v%2E113a49%2E488%2049%2E488%200%2000%2D6%200v%2D%2E113c0%2D%2E794%2E609%2D1%2E428%201%2E364%2D1%2E452zm%2D%2E355%205%2E945a%2E75%2E75%200%2010%2D1%2E5%2E058l%2E347%209a%2E75%2E75%200%20101%2E499%2D%2E058l%2D%2E346%2D9zm5%2E48%2E058a%2E75%2E75%200%2010%2D1%2E498%2D%2E058l%2D%2E347%209a%2E75%2E75%200%20001%2E5%2E058l%2E345%2D9z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TrashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 013.878.512.75.75 0 11-.256 1.478l-.209-.035-1.005 13.07a3 3 0 01-2.991 2.77H8.084a3 3 0 01-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 01-.256-1.478A48.567 48.567 0 017.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 013.369 0c1.603.051 2.815 1.387 2.815 2.951zm-6.136-1.452a51.196 51.196 0 013.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 00-6 0v-.113c0-.794.609-1.428 1.364-1.452zm-.355 5.945a.75.75 0 10-1.5.058l.347 9a.75.75 0 101.499-.058l-.346-9zm5.48.058a.75.75 0 10-1.498-.058l-.347 9a.75.75 0 001.5.058l.345-9z" clip-rule="evenodd"/>
</svg>
  }
}