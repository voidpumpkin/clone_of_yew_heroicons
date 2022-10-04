use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M11%2E828%202%2E25c%2D%2E916%200%2D1%2E699%2E663%2D1%2E85%201%2E567l%2D%2E091%2E549a%2E798%2E798%200%2001%2D%2E517%2E608%207%2E45%207%2E45%200%2000%2D%2E478%2E198%2E798%2E798%200%2001%2D%2E796%2D%2E064l%2D%2E453%2D%2E324a1%2E875%201%2E875%200%2000%2D2%2E416%2E2l%2D%2E243%2E243a1%2E875%201%2E875%200%2000%2D%2E2%202%2E416l%2E324%2E453a%2E798%2E798%200%2001%2E064%2E796%207%2E448%207%2E448%200%2000%2D%2E198%2E478%2E798%2E798%200%2001%2D%2E608%2E517l%2D%2E55%2E092a1%2E875%201%2E875%200%2000%2D1%2E566%201%2E849v%2E344c0%20%2E916%2E663%201%2E699%201%2E567%201%2E85l%2E549%2E091c%2E281%2E047%2E508%2E25%2E608%2E517%2E06%2E162%2E127%2E321%2E198%2E478a%2E798%2E798%200%2001%2D%2E064%2E796l%2D%2E324%2E453a1%2E875%201%2E875%200%2000%2E2%202%2E416l%2E243%2E243c%2E648%2E648%201%2E67%2E733%202%2E416%2E2l%2E453%2D%2E324a%2E798%2E798%200%2001%2E796%2D%2E064c%2E157%2E071%2E316%2E137%2E478%2E198%2E267%2E1%2E47%2E327%2E517%2E608l%2E092%2E55c%2E15%2E903%2E932%201%2E566%201%2E849%201%2E566h%2E344c%2E916%200%201%2E699%2D%2E663%201%2E85%2D1%2E567l%2E091%2D%2E549a%2E798%2E798%200%2001%2E517%2D%2E608%207%2E52%207%2E52%200%2000%2E478%2D%2E198%2E798%2E798%200%2001%2E796%2E064l%2E453%2E324a1%2E875%201%2E875%200%20002%2E416%2D%2E2l%2E243%2D%2E243c%2E648%2D%2E648%2E733%2D1%2E67%2E2%2D2%2E416l%2D%2E324%2D%2E453a%2E798%2E798%200%2001%2D%2E064%2D%2E796c%2E071%2D%2E157%2E137%2D%2E316%2E198%2D%2E478%2E1%2D%2E267%2E327%2D%2E47%2E608%2D%2E517l%2E55%2D%2E091a1%2E875%201%2E875%200%20001%2E566%2D1%2E85v%2D%2E344c0%2D%2E916%2D%2E663%2D1%2E699%2D1%2E567%2D1%2E85l%2D%2E549%2D%2E091a%2E798%2E798%200%2001%2D%2E608%2D%2E517%207%2E507%207%2E507%200%2000%2D%2E198%2D%2E478%2E798%2E798%200%2001%2E064%2D%2E796l%2E324%2D%2E453a1%2E875%201%2E875%200%2000%2D%2E2%2D2%2E416l%2D%2E243%2D%2E243a1%2E875%201%2E875%200%2000%2D2%2E416%2D%2E2l%2D%2E453%2E324a%2E798%2E798%200%2001%2D%2E796%2E064%207%2E462%207%2E462%200%2000%2D%2E478%2D%2E198%2E798%2E798%200%2001%2D%2E517%2D%2E608l%2D%2E091%2D%2E55a1%2E875%201%2E875%200%2000%2D1%2E85%2D1%2E566h%2D%2E344zM12%2015%2E75a3%2E75%203%2E75%200%20100%2D7%2E5%203%2E75%203%2E75%200%20000%207%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Cog8ToothIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M11.828 2.25c-.916 0-1.699.663-1.85 1.567l-.091.549a.798.798 0 01-.517.608 7.45 7.45 0 00-.478.198.798.798 0 01-.796-.064l-.453-.324a1.875 1.875 0 00-2.416.2l-.243.243a1.875 1.875 0 00-.2 2.416l.324.453a.798.798 0 01.064.796 7.448 7.448 0 00-.198.478.798.798 0 01-.608.517l-.55.092a1.875 1.875 0 00-1.566 1.849v.344c0 .916.663 1.699 1.567 1.85l.549.091c.281.047.508.25.608.517.06.162.127.321.198.478a.798.798 0 01-.064.796l-.324.453a1.875 1.875 0 00.2 2.416l.243.243c.648.648 1.67.733 2.416.2l.453-.324a.798.798 0 01.796-.064c.157.071.316.137.478.198.267.1.47.327.517.608l.092.55c.15.903.932 1.566 1.849 1.566h.344c.916 0 1.699-.663 1.85-1.567l.091-.549a.798.798 0 01.517-.608 7.52 7.52 0 00.478-.198.798.798 0 01.796.064l.453.324a1.875 1.875 0 002.416-.2l.243-.243c.648-.648.733-1.67.2-2.416l-.324-.453a.798.798 0 01-.064-.796c.071-.157.137-.316.198-.478.1-.267.327-.47.608-.517l.55-.091a1.875 1.875 0 001.566-1.85v-.344c0-.916-.663-1.699-1.567-1.85l-.549-.091a.798.798 0 01-.608-.517 7.507 7.507 0 00-.198-.478.798.798 0 01.064-.796l.324-.453a1.875 1.875 0 00-.2-2.416l-.243-.243a1.875 1.875 0 00-2.416-.2l-.453.324a.798.798 0 01-.796.064 7.462 7.462 0 00-.478-.198.798.798 0 01-.517-.608l-.091-.55a1.875 1.875 0 00-1.85-1.566h-.344zM12 15.75a3.75 3.75 0 100-7.5 3.75 3.75 0 000 7.5z" clip-rule="evenodd"/>
</svg>
  }
}
