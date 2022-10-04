use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E5%205%2E25a3%203%200%20013%2D3h3a3%203%200%20013%203v%2E205c%2E933%2E085%201%2E857%2E197%202%2E774%2E334%201%2E454%2E218%202%2E476%201%2E483%202%2E476%202%2E917v3%2E033c0%201%2E211%2D%2E734%202%2E352%2D1%2E936%202%2E752A24%2E726%2024%2E726%200%200112%2015%2E75c%2D2%2E73%200%2D5%2E357%2D%2E442%2D7%2E814%2D1%2E259%2D1%2E202%2D%2E4%2D1%2E936%2D1%2E541%2D1%2E936%2D2%2E752V8%2E706c0%2D1%2E434%201%2E022%2D2%2E7%202%2E476%2D2%2E917A48%2E814%2048%2E814%200%20017%2E5%205%2E455V5%2E25zm7%2E5%200v%2E09a49%2E488%2049%2E488%200%2000%2D6%200v%2D%2E09a1%2E5%201%2E5%200%20011%2E5%2D1%2E5h3a1%2E5%201%2E5%200%20011%2E5%201%2E5zm%2D3%208%2E25a%2E75%2E75%200%20100%2D1%2E5%2E75%2E75%200%20000%201%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M3%2018%2E4v%2D2%2E796a4%2E3%204%2E3%200%2000%2E713%2E31A26%2E226%2026%2E226%200%200012%2017%2E25c2%2E892%200%205%2E68%2D%2E468%208%2E287%2D1%2E335%2E252%2D%2E084%2E49%2D%2E189%2E713%2D%2E311V18%2E4c0%201%2E452%2D1%2E047%202%2E728%2D2%2E523%202%2E923%2D2%2E12%2E282%2D4%2E282%2E427%2D6%2E477%2E427a49%2E19%2049%2E19%200%2001%2D6%2E477%2D%2E427C4%2E047%2021%2E128%203%2019%2E852%203%2018%2E4z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BriefcaseIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M7.5 5.25a3 3 0 013-3h3a3 3 0 013 3v.205c.933.085 1.857.197 2.774.334 1.454.218 2.476 1.483 2.476 2.917v3.033c0 1.211-.734 2.352-1.936 2.752A24.726 24.726 0 0112 15.75c-2.73 0-5.357-.442-7.814-1.259-1.202-.4-1.936-1.541-1.936-2.752V8.706c0-1.434 1.022-2.7 2.476-2.917A48.814 48.814 0 017.5 5.455V5.25zm7.5 0v.09a49.488 49.488 0 00-6 0v-.09a1.5 1.5 0 011.5-1.5h3a1.5 1.5 0 011.5 1.5zm-3 8.25a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd"/>
  <path d="M3 18.4v-2.796a4.3 4.3 0 00.713.31A26.226 26.226 0 0012 17.25c2.892 0 5.68-.468 8.287-1.335.252-.084.49-.189.713-.311V18.4c0 1.452-1.047 2.728-2.523 2.923-2.12.282-4.282.427-6.477.427a49.19 49.19 0 01-6.477-.427C4.047 21.128 3 19.852 3 18.4z"/>
</svg>
  }
}