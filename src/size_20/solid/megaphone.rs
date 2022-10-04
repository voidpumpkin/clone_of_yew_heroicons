use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M13%2E92%203%2E845a19%2E361%2019%2E361%200%2001%2D6%2E3%201%2E98C6%2E765%205%2E942%205%2E89%206%205%206a4%204%200%2000%2D%2E504%207%2E969%2015%2E974%2015%2E974%200%20001%2E271%203%2E341c%2E397%2E77%201%2E342%201%202%2E05%2E59l%2E867%2D%2E5c%2E726%2D%2E42%2E94%2D1%2E321%2E588%2D2%2E021%2D%2E166%2D%2E33%2D%2E315%2D%2E666%2D%2E448%2D1%2E004%201%2E8%2E358%203%2E511%2E964%205%2E096%201%2E78A17%2E964%2017%2E964%200%200015%2010c0%2D2%2E161%2D%2E381%2D4%2E234%2D1%2E08%2D6%2E155zM15%2E243%203%2E097A19%2E456%2019%2E456%200%200116%2E5%2010c0%202%2E431%2D%2E445%204%2E758%2D1%2E257%206%2E904l%2D%2E03%2E077a%2E75%2E75%200%20001%2E401%2E537%2020%2E902%2020%2E902%200%20001%2E312%2D5%2E745%201%2E999%201%2E999%200%20000%2D3%2E545%2020%2E902%2020%2E902%200%2000%2D1%2E312%2D5%2E745%2E75%2E75%200%2000%2D1%2E4%2E537l%2E029%2E077z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MegaphoneIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M13.92 3.845a19.361 19.361 0 01-6.3 1.98C6.765 5.942 5.89 6 5 6a4 4 0 00-.504 7.969 15.974 15.974 0 001.271 3.341c.397.77 1.342 1 2.05.59l.867-.5c.726-.42.94-1.321.588-2.021-.166-.33-.315-.666-.448-1.004 1.8.358 3.511.964 5.096 1.78A17.964 17.964 0 0015 10c0-2.161-.381-4.234-1.08-6.155zM15.243 3.097A19.456 19.456 0 0116.5 10c0 2.431-.445 4.758-1.257 6.904l-.03.077a.75.75 0 001.401.537 20.902 20.902 0 001.312-5.745 1.999 1.999 0 000-3.545 20.902 20.902 0 00-1.312-5.745.75.75 0 00-1.4.537l.029.077z"/>
</svg>
  }
}
