use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%202%2E5c%2D1%2E31%200%2D2%2E526%2E386%2D3%2E546%201%2E051a%2E75%2E75%200%2001%2D%2E82%2D1%2E256A8%208%200%200118%209a22%2E47%2022%2E47%200%2001%2D1%2E228%207%2E351%2E75%2E75%200%2011%2D1%2E417%2D%2E49A20%2E97%2020%2E97%200%200016%2E5%209%206%2E5%206%2E5%200%200010%202%2E5zM4%2E333%204%2E416a%2E75%2E75%200%2001%2E218%201%2E038A6%2E466%206%2E466%200%20003%2E5%209a7%2E966%207%2E966%200%2001%2D1%2E293%204%2E362%2E75%2E75%200%2001%2D1%2E257%2D%2E819A6%2E466%206%2E466%200%20002%209c0%2D1%2E61%2E476%2D3%2E11%201%2E295%2D4%2E365a%2E75%2E75%200%20011%2E038%2D%2E219zM10%206%2E12a3%203%200%2000%2D3%2E001%203%2E041%2011%2E455%2011%2E455%200%2001%2D2%2E697%207%2E24%2E75%2E75%200%2001%2D1%2E148%2D%2E965A9%2E957%209%2E957%200%20005%2E5%209c0%2D%2E028%2E002%2D%2E055%2E004%2D%2E082a4%2E5%204%2E5%200%20018%2E996%2E084V9%2E15l%2D%2E005%2E297a%2E75%2E75%200%2011%2D1%2E5%2D%2E034c%2E003%2D%2E11%2E004%2D%2E219%2E005%2D%2E328a3%203%200%2000%2D3%2D2%2E965zm0%202%2E13a%2E75%2E75%200%2001%2E75%2E75c0%203%2E51%2D1%2E187%206%2E745%2D3%2E181%209%2E323a%2E75%2E75%200%2011%2D1%2E186%2D%2E918A13%2E687%2013%2E687%200%20009%2E25%209a%2E75%2E75%200%2001%2E75%2D%2E75zm3%2E529%203%2E698a%2E75%2E75%200%2001%2E584%2E885%2018%2E883%2018%2E883%200%2001%2D2%2E257%205%2E84%2E75%2E75%200%2011%2D1%2E29%2D%2E764%2017%2E386%2017%2E386%200%20002%2E078%2D5%2E377%2E75%2E75%200%2001%2E885%2D%2E584z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn FingerPrintIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 2.5c-1.31 0-2.526.386-3.546 1.051a.75.75 0 01-.82-1.256A8 8 0 0118 9a22.47 22.47 0 01-1.228 7.351.75.75 0 11-1.417-.49A20.97 20.97 0 0016.5 9 6.5 6.5 0 0010 2.5zM4.333 4.416a.75.75 0 01.218 1.038A6.466 6.466 0 003.5 9a7.966 7.966 0 01-1.293 4.362.75.75 0 01-1.257-.819A6.466 6.466 0 002 9c0-1.61.476-3.11 1.295-4.365a.75.75 0 011.038-.219zM10 6.12a3 3 0 00-3.001 3.041 11.455 11.455 0 01-2.697 7.24.75.75 0 01-1.148-.965A9.957 9.957 0 005.5 9c0-.028.002-.055.004-.082a4.5 4.5 0 018.996.084V9.15l-.005.297a.75.75 0 11-1.5-.034c.003-.11.004-.219.005-.328a3 3 0 00-3-2.965zm0 2.13a.75.75 0 01.75.75c0 3.51-1.187 6.745-3.181 9.323a.75.75 0 11-1.186-.918A13.687 13.687 0 009.25 9a.75.75 0 01.75-.75zm3.529 3.698a.75.75 0 01.584.885 18.883 18.883 0 01-2.257 5.84.75.75 0 11-1.29-.764 17.386 17.386 0 002.078-5.377.75.75 0 01.885-.584z" clip-rule="evenodd"/>
</svg>
  }
}
