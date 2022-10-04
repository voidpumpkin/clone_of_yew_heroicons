use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M15%2E73%205%2E25h1%2E035A7%2E465%207%2E465%200%200118%209%2E375a7%2E465%207%2E465%200%2001%2D1%2E235%204%2E125h%2D%2E148c%2D%2E806%200%2D1%2E534%2E446%2D2%2E031%201%2E08a9%2E04%209%2E04%200%2001%2D2%2E861%202%2E4c%2D%2E723%2E384%2D1%2E35%2E956%2D1%2E653%201%2E715a4%2E498%204%2E498%200%2000%2D%2E322%201%2E672V21a%2E75%2E75%200%2001%2D%2E75%2E75%202%2E25%202%2E25%200%2001%2D2%2E25%2D2%2E25c0%2D1%2E152%2E26%2D2%2E243%2E723%2D3%2E218C7%2E74%2015%2E724%207%2E366%2015%206%2E748%2015H3%2E622c%2D1%2E026%200%2D1%2E945%2D%2E694%2D2%2E054%2D1%2E715A12%2E134%2012%2E134%200%20011%2E5%2012c0%2D2%2E848%2E992%2D5%2E464%202%2E649%2D7%2E521%2E388%2D%2E482%2E987%2D%2E729%201%2E605%2D%2E729H9%2E77a4%2E5%204%2E5%200%20011%2E423%2E23l3%2E114%201%2E04a4%2E5%204%2E5%200%20001%2E423%2E23zM21%2E669%2013%2E773c%2E536%2D1%2E362%2E831%2D2%2E845%2E831%2D4%2E398%200%2D1%2E22%2D%2E182%2D2%2E398%2D%2E52%2D3%2E507%2D%2E26%2D%2E85%2D1%2E084%2D1%2E368%2D1%2E973%2D1%2E368H19%2E1c%2D%2E445%200%2D%2E72%2E498%2D%2E523%2E898%2E591%201%2E2%2E924%202%2E55%2E924%203%2E977a8%2E959%208%2E959%200%2001%2D1%2E302%204%2E666c%2D%2E245%2E403%2E028%2E959%2E5%2E959h1%2E053c%2E832%200%201%2E612%2D%2E453%201%2E918%2D1%2E227z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HandThumbDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M15.73 5.25h1.035A7.465 7.465 0 0118 9.375a7.465 7.465 0 01-1.235 4.125h-.148c-.806 0-1.534.446-2.031 1.08a9.04 9.04 0 01-2.861 2.4c-.723.384-1.35.956-1.653 1.715a4.498 4.498 0 00-.322 1.672V21a.75.75 0 01-.75.75 2.25 2.25 0 01-2.25-2.25c0-1.152.26-2.243.723-3.218C7.74 15.724 7.366 15 6.748 15H3.622c-1.026 0-1.945-.694-2.054-1.715A12.134 12.134 0 011.5 12c0-2.848.992-5.464 2.649-7.521.388-.482.987-.729 1.605-.729H9.77a4.5 4.5 0 011.423.23l3.114 1.04a4.5 4.5 0 001.423.23zM21.669 13.773c.536-1.362.831-2.845.831-4.398 0-1.22-.182-2.398-.52-3.507-.26-.85-1.084-1.368-1.973-1.368H19.1c-.445 0-.72.498-.523.898.591 1.2.924 2.55.924 3.977a8.959 8.959 0 01-1.302 4.666c-.245.403.028.959.5.959h1.053c.832 0 1.612-.453 1.918-1.227z"/>
</svg>
  }
}
