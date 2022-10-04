use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%205%2E25c1%2E213%200%202%2E415%2E046%203%2E605%2E135a3%2E256%203%2E256%200%20013%2E01%203%2E01c%2E044%2E583%2E077%201%2E17%2E1%201%2E759L17%2E03%208%2E47a%2E75%2E75%200%2010%2D1%2E06%201%2E06l3%203a%2E75%2E75%200%20001%2E06%200l3%2D3a%2E75%2E75%200%2000%2D1%2E06%2D1%2E06l%2D1%2E752%201%2E751c%2D%2E023%2D%2E65%2D%2E06%2D1%2E296%2D%2E108%2D1%2E939a4%2E756%204%2E756%200%2000%2D4%2E392%2D4%2E392%2049%2E422%2049%2E422%200%2000%2D7%2E436%200A4%2E756%204%2E756%200%20003%2E89%208%2E282c%2D%2E017%2E224%2D%2E033%2E447%2D%2E046%2E672a%2E75%2E75%200%20101%2E497%2E092c%2E013%2D%2E217%2E028%2D%2E434%2E044%2D%2E651a3%2E256%203%2E256%200%20013%2E01%2D3%2E01c1%2E19%2D%2E09%202%2E392%2D%2E135%203%2E605%2D%2E135zm%2D6%2E97%206%2E22a%2E75%2E75%200%2000%2D1%2E06%200l%2D3%203a%2E75%2E75%200%20101%2E06%201%2E06l1%2E752%2D1%2E751c%2E023%2E65%2E06%201%2E296%2E108%201%2E939a4%2E756%204%2E756%200%20004%2E392%204%2E392%2049%2E413%2049%2E413%200%20007%2E436%200%204%2E756%204%2E756%200%20004%2E392%2D4%2E392c%2E017%2D%2E223%2E032%2D%2E447%2E046%2D%2E672a%2E75%2E75%200%2000%2D1%2E497%2D%2E092c%2D%2E013%2E217%2D%2E028%2E434%2D%2E044%2E651a3%2E256%203%2E256%200%2001%2D3%2E01%203%2E01%2047%2E953%2047%2E953%200%2001%2D7%2E21%200%203%2E256%203%2E256%200%2001%2D3%2E01%2D3%2E01%2047%2E759%2047%2E759%200%2001%2D%2E1%2D1%2E759L6%2E97%2015%2E53a%2E75%2E75%200%20001%2E06%2D1%2E06l%2D3%2D3z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowPathRoundedSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 5.25c1.213 0 2.415.046 3.605.135a3.256 3.256 0 013.01 3.01c.044.583.077 1.17.1 1.759L17.03 8.47a.75.75 0 10-1.06 1.06l3 3a.75.75 0 001.06 0l3-3a.75.75 0 00-1.06-1.06l-1.752 1.751c-.023-.65-.06-1.296-.108-1.939a4.756 4.756 0 00-4.392-4.392 49.422 49.422 0 00-7.436 0A4.756 4.756 0 003.89 8.282c-.017.224-.033.447-.046.672a.75.75 0 101.497.092c.013-.217.028-.434.044-.651a3.256 3.256 0 013.01-3.01c1.19-.09 2.392-.135 3.605-.135zm-6.97 6.22a.75.75 0 00-1.06 0l-3 3a.75.75 0 101.06 1.06l1.752-1.751c.023.65.06 1.296.108 1.939a4.756 4.756 0 004.392 4.392 49.413 49.413 0 007.436 0 4.756 4.756 0 004.392-4.392c.017-.223.032-.447.046-.672a.75.75 0 00-1.497-.092c-.013.217-.028.434-.044.651a3.256 3.256 0 01-3.01 3.01 47.953 47.953 0 01-7.21 0 3.256 3.256 0 01-3.01-3.01 47.759 47.759 0 01-.1-1.759L6.97 15.53a.75.75 0 001.06-1.06l-3-3z" clip-rule="evenodd"/>
</svg>
  }
}