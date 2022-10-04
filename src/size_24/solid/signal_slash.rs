use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E47%202%2E47a%2E75%2E75%200%20011%2E06%200l8%2E407%208%2E407a1%2E125%201%2E125%200%20011%2E186%201%2E186l1%2E462%201%2E461a3%2E001%203%2E001%200%2000%2D%2E464%2D3%2E645%2E75%2E75%200%20111%2E061%2D1%2E061%204%2E501%204%2E501%200%2001%2E486%205%2E79l1%2E072%201%2E072a6%2E001%206%2E001%200%2000%2D%2E497%2D7%2E923%2E75%2E75%200%20011%2E06%2D1%2E06%207%2E501%207%2E501%200%2001%2E505%2010%2E05l1%2E064%201%2E065a9%209%200%2000%2D%2E508%2D12%2E176%2E75%2E75%200%20011%2E06%2D1%2E06c3%2E923%203%2E922%204%2E093%2010%2E175%2E512%2014%2E3l1%2E594%201%2E594a%2E75%2E75%200%2011%2D1%2E06%201%2E06l%2D2%2E106%2D2%2E105%2D2%2E121%2D2%2E122h%2D%2E001l%2D4%2E705%2D4%2E706a%2E747%2E747%200%2001%2D%2E127%2D%2E126L2%2E47%203%2E53a%2E75%2E75%200%20010%2D1%2E061zm1%2E189%204%2E422a%2E75%2E75%200%2001%2E326%201%2E01%209%2E004%209%2E004%200%20001%2E651%2010%2E462%2E75%2E75%200%2011%2D1%2E06%201%2E06C1%2E27%2016%2E12%2E63%2011%2E165%202%2E648%207%2E219a%2E75%2E75%200%20011%2E01%2D%2E326zM5%2E84%209%2E134a%2E75%2E75%200%2001%2E472%2E95%206%206%200%20001%2E444%206%2E159%2E75%2E75%200%2001%2D1%2E06%201%2E06A7%2E5%207%2E5%200%20014%2E89%209%2E606a%2E75%2E75%200%2001%2E95%2D%2E472zm2%2E341%202%2E653a%2E75%2E75%200%2001%2E848%2E638c%2E088%2E62%2E37%201%2E218%2E849%201%2E696a%2E75%2E75%200%2001%2D1%2E061%201%2E061%204%2E483%204%2E483%200%2001%2D1%2E273%2D2%2E546%2E75%2E75%200%2001%2E637%2D%2E848z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SignalSlashIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.47 2.47a.75.75 0 011.06 0l8.407 8.407a1.125 1.125 0 011.186 1.186l1.462 1.461a3.001 3.001 0 00-.464-3.645.75.75 0 111.061-1.061 4.501 4.501 0 01.486 5.79l1.072 1.072a6.001 6.001 0 00-.497-7.923.75.75 0 011.06-1.06 7.501 7.501 0 01.505 10.05l1.064 1.065a9 9 0 00-.508-12.176.75.75 0 011.06-1.06c3.923 3.922 4.093 10.175.512 14.3l1.594 1.594a.75.75 0 11-1.06 1.06l-2.106-2.105-2.121-2.122h-.001l-4.705-4.706a.747.747 0 01-.127-.126L2.47 3.53a.75.75 0 010-1.061zm1.189 4.422a.75.75 0 01.326 1.01 9.004 9.004 0 001.651 10.462.75.75 0 11-1.06 1.06C1.27 16.12.63 11.165 2.648 7.219a.75.75 0 011.01-.326zM5.84 9.134a.75.75 0 01.472.95 6 6 0 001.444 6.159.75.75 0 01-1.06 1.06A7.5 7.5 0 014.89 9.606a.75.75 0 01.95-.472zm2.341 2.653a.75.75 0 01.848.638c.088.62.37 1.218.849 1.696a.75.75 0 01-1.061 1.061 4.483 4.483 0 01-1.273-2.546.75.75 0 01.637-.848z" clip-rule="evenodd"/>
</svg>
  }
}