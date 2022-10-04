use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M20%2E25%2014%2E15v4%2E25c0%201%2E094%2D%2E787%202%2E036%2D1%2E872%202%2E18%2D2%2E087%2E277%2D4%2E216%2E42%2D6%2E378%2E42s%2D4%2E291%2D%2E143%2D6%2E378%2D%2E42c%2D1%2E085%2D%2E144%2D1%2E872%2D1%2E086%2D1%2E872%2D2%2E18v%2D4%2E25m16%2E5%200a2%2E18%202%2E18%200%2000%2E75%2D1%2E661V8%2E706c0%2D1%2E081%2D%2E768%2D2%2E015%2D1%2E837%2D2%2E175a48%2E114%2048%2E114%200%2000%2D3%2E413%2D%2E387m4%2E5%208%2E006c%2D%2E194%2E165%2D%2E42%2E295%2D%2E673%2E38A23%2E978%2023%2E978%200%200112%2015%2E75c%2D2%2E648%200%2D5%2E195%2D%2E429%2D7%2E577%2D1%2E22a2%2E016%202%2E016%200%2001%2D%2E673%2D%2E38m0%200A2%2E18%202%2E18%200%20013%2012%2E489V8%2E706c0%2D1%2E081%2E768%2D2%2E015%201%2E837%2D2%2E175a48%2E111%2048%2E111%200%20013%2E413%2D%2E387m7%2E5%200V5%2E25A2%2E25%202%2E25%200%200013%2E5%203h%2D3a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25v%2E894m7%2E5%200a48%2E667%2048%2E667%200%2000%2D7%2E5%200M12%2012%2E75h%2E008v%2E008H12v%2D%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BriefcaseIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 14.15v4.25c0 1.094-.787 2.036-1.872 2.18-2.087.277-4.216.42-6.378.42s-4.291-.143-6.378-.42c-1.085-.144-1.872-1.086-1.872-2.18v-4.25m16.5 0a2.18 2.18 0 00.75-1.661V8.706c0-1.081-.768-2.015-1.837-2.175a48.114 48.114 0 00-3.413-.387m4.5 8.006c-.194.165-.42.295-.673.38A23.978 23.978 0 0112 15.75c-2.648 0-5.195-.429-7.577-1.22a2.016 2.016 0 01-.673-.38m0 0A2.18 2.18 0 013 12.489V8.706c0-1.081.768-2.015 1.837-2.175a48.111 48.111 0 013.413-.387m7.5 0V5.25A2.25 2.25 0 0013.5 3h-3a2.25 2.25 0 00-2.25 2.25v.894m7.5 0a48.667 48.667 0 00-7.5 0M12 12.75h.008v.008H12v-.008z"/>
</svg>
  }
}
