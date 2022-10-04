use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M20%2E432%204%2E103a%2E75%2E75%200%2000%2D%2E364%2D1%2E455L4%2E128%206%2E632l%2D%2E2%2E033C2%2E498%206%2E904%201%2E5%208%2E158%201%2E5%209%2E575v9%2E175a3%203%200%20003%203h15a3%203%200%20003%2D3V9%2E574c0%2D1%2E416%2D%2E997%2D2%2E67%2D2%2E429%2D2%2E909a49%2E016%2049%2E016%200%2000%2D7%2E255%2D%2E658l7%2E616%2D1%2E904zm%2D9%2E585%208%2E56a%2E75%2E75%200%20010%201%2E06l%2D%2E005%2E006a%2E75%2E75%200%2001%2D1%2E06%200l%2D%2E006%2D%2E005a%2E75%2E75%200%20010%2D1%2E061l%2E005%2D%2E005a%2E75%2E75%200%20011%2E06%200l%2E006%2E005zM9%2E781%2015%2E85a%2E75%2E75%200%20001%2E061%200l%2E005%2D%2E005a%2E75%2E75%200%20000%2D1%2E061l%2D%2E005%2D%2E005a%2E75%2E75%200%2000%2D1%2E06%200l%2D%2E006%2E005a%2E75%2E75%200%20000%201%2E06l%2E005%2E006zm%2D1%2E055%2D1%2E066a%2E75%2E75%200%20010%201%2E06l%2D%2E005%2E006a%2E75%2E75%200%2001%2D1%2E061%200l%2D%2E005%2D%2E005a%2E75%2E75%200%20010%2D1%2E06l%2E005%2D%2E006a%2E75%2E75%200%20011%2E06%200l%2E006%2E005zM7%2E66%2013%2E73a%2E75%2E75%200%20001%2E061%200l%2E005%2D%2E006a%2E75%2E75%200%20000%2D1%2E06l%2D%2E005%2D%2E005a%2E75%2E75%200%2000%2D1%2E06%200l%2D%2E006%2E005a%2E75%2E75%200%20000%201%2E06l%2E005%2E006zM9%2E255%209%2E75a%2E75%2E75%200%2001%2E75%2E75v%2E008a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E008a%2E75%2E75%200%2001%2D%2E75%2D%2E75V10%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h%2E008zm3%2E624%203%2E28a%2E75%2E75%200%2000%2E275%2D1%2E025L13%2E15%2012a%2E75%2E75%200%2000%2D1%2E025%2D%2E275l%2D%2E006%2E004a%2E75%2E75%200%2000%2D%2E275%201%2E024l%2E004%2E007a%2E75%2E75%200%20001%2E024%2E274l%2E007%2D%2E003zm%2D1%2E38%205%2E126a%2E75%2E75%200%2001%2D1%2E024%2D%2E274l%2D%2E004%2D%2E007a%2E75%2E75%200%2001%2E275%2D1%2E024l%2E006%2D%2E004a%2E75%2E75%200%20011%2E025%2E274l%2E004%2E007a%2E75%2E75%200%2001%2D%2E275%201%2E024l%2D%2E006%2E004zm%2E282%2D6%2E776a%2E75%2E75%200%2000%2D%2E274%2D1%2E025l%2D%2E007%2D%2E003a%2E75%2E75%200%2000%2D1%2E024%2E274l%2D%2E004%2E007a%2E75%2E75%200%2000%2E274%201%2E024l%2E007%2E004a%2E75%2E75%200%20001%2E024%2D%2E274l%2E004%2D%2E007zm1%2E369%205%2E129a%2E75%2E75%200%2001%2D1%2E025%2E274l%2D%2E006%2D%2E003a%2E75%2E75%200%2001%2D%2E275%2D1%2E025l%2E004%2D%2E006a%2E75%2E75%200%20011%2E025%2D%2E275l%2E006%2E004a%2E75%2E75%200%2001%2E275%201%2E024l%2D%2E004%2E007zm%2D%2E145%2D1%2E502a%2E75%2E75%200%2000%2E75%2D%2E75v%2D%2E007a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D%2E008a%2E75%2E75%200%2000%2D%2E75%2E75v%2E008c0%20%2E414%2E336%2E75%2E75%2E75h%2E008zm%2D3%2E75%202%2E243a%2E75%2E75%200%2001%2E75%2E75v%2E008a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E008a%2E75%2E75%200%2001%2D%2E75%2D%2E75V18a%2E75%2E75%200%2001%2E75%2D%2E75h%2E008zm%2D2%2E871%2D%2E47a%2E75%2E75%200%2000%2E274%2D1%2E025l%2D%2E003%2D%2E006a%2E75%2E75%200%2000%2D1%2E025%2D%2E275l%2D%2E006%2E004a%2E75%2E75%200%2000%2D%2E275%201%2E025l%2E004%2E006a%2E75%2E75%200%20001%2E024%2E274l%2E007%2D%2E003zm1%2E366%2D5%2E12a%2E75%2E75%200%2001%2D1%2E025%2D%2E274l%2D%2E004%2D%2E006a%2E75%2E75%200%2001%2E275%2D1%2E025l%2E006%2D%2E003a%2E75%2E75%200%20011%2E025%2E274l%2E004%2E007a%2E75%2E75%200%2001%2D%2E275%201%2E024l%2D%2E006%2E004zm%2E281%206%2E215a%2E75%2E75%200%2000%2D%2E275%2D1%2E024l%2D%2E006%2D%2E004a%2E75%2E75%200%2000%2D1%2E025%2E274l%2D%2E003%2E007a%2E75%2E75%200%2000%2E274%201%2E024l%2E007%2E004a%2E75%2E75%200%20001%2E024%2D%2E274l%2E004%2D%2E007zM6%2E655%2012%2E76a%2E75%2E75%200%2001%2D1%2E025%2E274l%2D%2E006%2D%2E003a%2E75%2E75%200%2001%2D%2E275%2D1%2E025L5%2E353%2012a%2E75%2E75%200%20011%2E025%2D%2E275l%2E006%2E004a%2E75%2E75%200%2001%2E275%201%2E024l%2D%2E004%2E007zm%2D1%2E15%202%2E248a%2E75%2E75%200%2000%2E75%2D%2E75v%2D%2E007a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D%2E008a%2E75%2E75%200%2000%2D%2E75%2E75v%2E008c0%20%2E414%2E336%2E75%2E75%2E75h%2E008zM17%2E25%2010%2E5a1%2E5%201%2E5%200%20110%203%201%2E5%201%2E5%200%20010%2D3zm1%2E5%206a1%2E5%201%2E5%200%2010%2D3%200%201%2E5%201%2E5%200%20003%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn RadioIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M20.432 4.103a.75.75 0 00-.364-1.455L4.128 6.632l-.2.033C2.498 6.904 1.5 8.158 1.5 9.575v9.175a3 3 0 003 3h15a3 3 0 003-3V9.574c0-1.416-.997-2.67-2.429-2.909a49.016 49.016 0 00-7.255-.658l7.616-1.904zm-9.585 8.56a.75.75 0 010 1.06l-.005.006a.75.75 0 01-1.06 0l-.006-.005a.75.75 0 010-1.061l.005-.005a.75.75 0 011.06 0l.006.005zM9.781 15.85a.75.75 0 001.061 0l.005-.005a.75.75 0 000-1.061l-.005-.005a.75.75 0 00-1.06 0l-.006.005a.75.75 0 000 1.06l.005.006zm-1.055-1.066a.75.75 0 010 1.06l-.005.006a.75.75 0 01-1.061 0l-.005-.005a.75.75 0 010-1.06l.005-.006a.75.75 0 011.06 0l.006.005zM7.66 13.73a.75.75 0 001.061 0l.005-.006a.75.75 0 000-1.06l-.005-.005a.75.75 0 00-1.06 0l-.006.005a.75.75 0 000 1.06l.005.006zM9.255 9.75a.75.75 0 01.75.75v.008a.75.75 0 01-.75.75h-.008a.75.75 0 01-.75-.75V10.5a.75.75 0 01.75-.75h.008zm3.624 3.28a.75.75 0 00.275-1.025L13.15 12a.75.75 0 00-1.025-.275l-.006.004a.75.75 0 00-.275 1.024l.004.007a.75.75 0 001.024.274l.007-.003zm-1.38 5.126a.75.75 0 01-1.024-.274l-.004-.007a.75.75 0 01.275-1.024l.006-.004a.75.75 0 011.025.274l.004.007a.75.75 0 01-.275 1.024l-.006.004zm.282-6.776a.75.75 0 00-.274-1.025l-.007-.003a.75.75 0 00-1.024.274l-.004.007a.75.75 0 00.274 1.024l.007.004a.75.75 0 001.024-.274l.004-.007zm1.369 5.129a.75.75 0 01-1.025.274l-.006-.003a.75.75 0 01-.275-1.025l.004-.006a.75.75 0 011.025-.275l.006.004a.75.75 0 01.275 1.024l-.004.007zm-.145-1.502a.75.75 0 00.75-.75v-.007a.75.75 0 00-.75-.75h-.008a.75.75 0 00-.75.75v.008c0 .414.336.75.75.75h.008zm-3.75 2.243a.75.75 0 01.75.75v.008a.75.75 0 01-.75.75h-.008a.75.75 0 01-.75-.75V18a.75.75 0 01.75-.75h.008zm-2.871-.47a.75.75 0 00.274-1.025l-.003-.006a.75.75 0 00-1.025-.275l-.006.004a.75.75 0 00-.275 1.025l.004.006a.75.75 0 001.024.274l.007-.003zm1.366-5.12a.75.75 0 01-1.025-.274l-.004-.006a.75.75 0 01.275-1.025l.006-.003a.75.75 0 011.025.274l.004.007a.75.75 0 01-.275 1.024l-.006.004zm.281 6.215a.75.75 0 00-.275-1.024l-.006-.004a.75.75 0 00-1.025.274l-.003.007a.75.75 0 00.274 1.024l.007.004a.75.75 0 001.024-.274l.004-.007zM6.655 12.76a.75.75 0 01-1.025.274l-.006-.003a.75.75 0 01-.275-1.025L5.353 12a.75.75 0 011.025-.275l.006.004a.75.75 0 01.275 1.024l-.004.007zm-1.15 2.248a.75.75 0 00.75-.75v-.007a.75.75 0 00-.75-.75h-.008a.75.75 0 00-.75.75v.008c0 .414.336.75.75.75h.008zM17.25 10.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zm1.5 6a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z" clip-rule="evenodd"/>
</svg>
  }
}