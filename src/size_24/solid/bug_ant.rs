use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M8%2E478%201%2E6a%2E75%2E75%200%2001%2E273%201%2E025%203%2E72%203%2E72%200%2000%2D%2E425%201%2E122c%2E058%2E057%2E118%2E114%2E18%2E168A4%2E491%204%2E491%200%200112%202%2E25c1%2E413%200%202%2E673%2E651%203%2E497%201%2E668%2E06%2D%2E054%2E12%2D%2E11%2E178%2D%2E167a3%2E717%203%2E717%200%2000%2D%2E426%2D1%2E126%2E75%2E75%200%20111%2E298%2D%2E75%205%2E22%205%2E22%200%2001%2E671%202%2E045%2E75%2E75%200%2001%2D%2E187%2E582c%2D%2E241%2E27%2D%2E505%2E52%2D%2E787%2E749a4%2E495%204%2E495%200%2001%2E216%202%2E1c%2D%2E106%2E792%2D%2E753%201%2E295%2D1%2E417%201%2E403%2D%2E182%2E03%2D%2E364%2E057%2D%2E547%2E081%2E152%2E227%2E273%2E476%2E359%2E741a23%2E122%2023%2E122%200%20003%2E832%2D%2E802%2023%2E241%2023%2E241%200%2000%2D%2E345%2D2%2E634%2E75%2E75%200%20011%2E474%2D%2E28c%2E21%201%2E115%2E348%202%2E256%2E404%203%2E418a%2E75%2E75%200%2001%2D%2E516%2E749c%2D1%2E527%2E5%2D3%2E119%2E855%2D4%2E76%201%2E05%2D%2E074%2E38%2D%2E22%2E735%2D%2E423%201%2E05a24%2E61%2024%2E61%200%20015%2E943%201%2E358%2E75%2E75%200%2001%2E492%2E75%2024%2E665%2024%2E665%200%2001%2D1%2E189%206%2E25%2E75%2E75%200%2001%2D1%2E425%2D%2E47%2023%2E141%2023%2E141%200%20001%2E077%2D5%2E307c%2D%2E5%2D%2E168%2D1%2E009%2D%2E32%2D1%2E524%2D%2E454%2E068%2E234%2E104%2E484%2E104%2E746%200%203%2E956%2D2%2E521%207%2E5%2D6%207%2E5%2D3%2E478%200%2D6%2D3%2E544%2D6%2D7%2E5%200%2D%2E262%2E037%2D%2E511%2E104%2D%2E746%2D%2E514%2E134%2D1%2E022%2E286%2D1%2E522%2E454a23%2E14%2023%2E14%200%20001%2E077%205%2E308%2E75%2E75%200%2001%2D1%2E425%2E468%2024%2E663%2024%2E663%200%2001%2D1%2E19%2D6%2E25%2E75%2E75%200%2001%2E493%2D%2E749%2024%2E593%2024%2E593%200%20014%2E964%2D1%2E24h%2E01c%2E321%2D%2E046%2E644%2D%2E085%2E969%2D%2E118a2%2E982%202%2E982%200%2001%2D%2E424%2D1%2E05%2024%2E614%2024%2E614%200%2001%2D4%2E76%2D1%2E05%2E75%2E75%200%2001%2D%2E516%2D%2E75c%2E057%2D1%2E161%2E194%2D2%2E302%2E405%2D3%2E417a%2E75%2E75%200%20011%2E474%2E28c%2D%2E164%2E862%2D%2E28%201%2E74%2D%2E345%202%2E634%201%2E237%2E37%202%2E517%2E641%203%2E832%2E802%2E085%2D%2E265%2E207%2D%2E514%2E359%2D%2E74a18%2E732%2018%2E732%200%2001%2D%2E547%2D%2E082c%2D%2E664%2D%2E108%2D1%2E311%2D%2E611%2D1%2E417%2D1%2E403a4%2E535%204%2E535%200%2001%2E217%2D2%2E103%206%2E788%206%2E788%200%2001%2D%2E788%2D%2E751%2E75%2E75%200%2001%2D%2E187%2D%2E583%205%2E22%205%2E22%200%2001%2E67%2D2%2E04%2E75%2E75%200%20011%2E026%2D%2E273z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BugAntIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M8.478 1.6a.75.75 0 01.273 1.025 3.72 3.72 0 00-.425 1.122c.058.057.118.114.18.168A4.491 4.491 0 0112 2.25c1.413 0 2.673.651 3.497 1.668.06-.054.12-.11.178-.167a3.717 3.717 0 00-.426-1.126.75.75 0 111.298-.75 5.22 5.22 0 01.671 2.045.75.75 0 01-.187.582c-.241.27-.505.52-.787.749a4.495 4.495 0 01.216 2.1c-.106.792-.753 1.295-1.417 1.403-.182.03-.364.057-.547.081.152.227.273.476.359.741a23.122 23.122 0 003.832-.802 23.241 23.241 0 00-.345-2.634.75.75 0 011.474-.28c.21 1.115.348 2.256.404 3.418a.75.75 0 01-.516.749c-1.527.5-3.119.855-4.76 1.05-.074.38-.22.735-.423 1.05a24.61 24.61 0 015.943 1.358.75.75 0 01.492.75 24.665 24.665 0 01-1.189 6.25.75.75 0 01-1.425-.47 23.141 23.141 0 001.077-5.307c-.5-.168-1.009-.32-1.524-.454.068.234.104.484.104.746 0 3.956-2.521 7.5-6 7.5-3.478 0-6-3.544-6-7.5 0-.262.037-.511.104-.746-.514.134-1.022.286-1.522.454a23.14 23.14 0 001.077 5.308.75.75 0 01-1.425.468 24.663 24.663 0 01-1.19-6.25.75.75 0 01.493-.749 24.593 24.593 0 014.964-1.24h.01c.321-.046.644-.085.969-.118a2.982 2.982 0 01-.424-1.05 24.614 24.614 0 01-4.76-1.05.75.75 0 01-.516-.75c.057-1.161.194-2.302.405-3.417a.75.75 0 011.474.28c-.164.862-.28 1.74-.345 2.634 1.237.37 2.517.641 3.832.802.085-.265.207-.514.359-.74a18.732 18.732 0 01-.547-.082c-.664-.108-1.311-.611-1.417-1.403a4.535 4.535 0 01.217-2.103 6.788 6.788 0 01-.788-.751.75.75 0 01-.187-.583 5.22 5.22 0 01.67-2.04.75.75 0 011.026-.273z" clip-rule="evenodd"/>
</svg>
  }
}