use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%206%2E042A8%2E967%208%2E967%200%20006%203%2E75c%2D1%2E052%200%2D2%2E062%2E18%2D3%20%2E512v14%2E25A8%2E987%208%2E987%200%20016%2018c2%2E305%200%204%2E408%2E867%206%202%2E292m0%2D14%2E25a8%2E966%208%2E966%200%20016%2D2%2E292c1%2E052%200%202%2E062%2E18%203%20%2E512v14%2E25A8%2E987%208%2E987%200%200018%2018a8%2E967%208%2E967%200%2000%2D6%202%2E292m0%2D14%2E25v14%2E25%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BookOpenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"/>
</svg>
  }
}
