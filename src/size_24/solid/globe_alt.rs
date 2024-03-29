use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M21%2E721%2012%2E752a9%2E711%209%2E711%200%2000%2D%2E945%2D5%2E003%2012%2E754%2012%2E754%200%2001%2D4%2E339%202%2E708%2018%2E991%2018%2E991%200%2001%2D%2E214%204%2E772%2017%2E165%2017%2E165%200%20005%2E498%2D2%2E477zM14%2E634%2015%2E55a17%2E324%2017%2E324%200%2000%2E332%2D4%2E647c%2D%2E952%2E227%2D1%2E945%2E347%2D2%2E966%2E347%2D1%2E021%200%2D2%2E014%2D%2E12%2D2%2E966%2D%2E347a17%2E515%2017%2E515%200%2000%2E332%204%2E647%2017%2E385%2017%2E385%200%20005%2E268%200zM9%2E772%2017%2E119a18%2E963%2018%2E963%200%20004%2E456%200A17%2E182%2017%2E182%200%200112%2021%2E724a17%2E18%2017%2E18%200%2001%2D2%2E228%2D4%2E605zM7%2E777%2015%2E23a18%2E87%2018%2E87%200%2001%2D%2E214%2D4%2E774%2012%2E753%2012%2E753%200%2001%2D4%2E34%2D2%2E708%209%2E711%209%2E711%200%2000%2D%2E944%205%2E004%2017%2E165%2017%2E165%200%20005%2E498%202%2E477zM21%2E356%2014%2E752a9%2E765%209%2E765%200%2001%2D7%2E478%206%2E817%2018%2E64%2018%2E64%200%20001%2E988%2D4%2E718%2018%2E627%2018%2E627%200%20005%2E49%2D2%2E098zM2%2E644%2014%2E752c1%2E682%2E971%203%2E53%201%2E688%205%2E49%202%2E099a18%2E64%2018%2E64%200%20001%2E988%204%2E718%209%2E765%209%2E765%200%2001%2D7%2E478%2D6%2E816zM13%2E878%202%2E43a9%2E755%209%2E755%200%20016%2E116%203%2E986%2011%2E267%2011%2E267%200%2001%2D3%2E746%202%2E504%2018%2E63%2018%2E63%200%2000%2D2%2E37%2D6%2E49zM12%202%2E276a17%2E152%2017%2E152%200%20012%2E805%207%2E121c%2D%2E897%2E23%2D1%2E837%2E353%2D2%2E805%2E353%2D%2E968%200%2D1%2E908%2D%2E122%2D2%2E805%2D%2E353A17%2E151%2017%2E151%200%200112%202%2E276zM10%2E122%202%2E43a18%2E629%2018%2E629%200%2000%2D2%2E37%206%2E49%2011%2E266%2011%2E266%200%2001%2D3%2E746%2D2%2E504%209%2E754%209%2E754%200%20016%2E116%2D3%2E985z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn GlobeAltIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M21.721 12.752a9.711 9.711 0 00-.945-5.003 12.754 12.754 0 01-4.339 2.708 18.991 18.991 0 01-.214 4.772 17.165 17.165 0 005.498-2.477zM14.634 15.55a17.324 17.324 0 00.332-4.647c-.952.227-1.945.347-2.966.347-1.021 0-2.014-.12-2.966-.347a17.515 17.515 0 00.332 4.647 17.385 17.385 0 005.268 0zM9.772 17.119a18.963 18.963 0 004.456 0A17.182 17.182 0 0112 21.724a17.18 17.18 0 01-2.228-4.605zM7.777 15.23a18.87 18.87 0 01-.214-4.774 12.753 12.753 0 01-4.34-2.708 9.711 9.711 0 00-.944 5.004 17.165 17.165 0 005.498 2.477zM21.356 14.752a9.765 9.765 0 01-7.478 6.817 18.64 18.64 0 001.988-4.718 18.627 18.627 0 005.49-2.098zM2.644 14.752c1.682.971 3.53 1.688 5.49 2.099a18.64 18.64 0 001.988 4.718 9.765 9.765 0 01-7.478-6.816zM13.878 2.43a9.755 9.755 0 016.116 3.986 11.267 11.267 0 01-3.746 2.504 18.63 18.63 0 00-2.37-6.49zM12 2.276a17.152 17.152 0 012.805 7.121c-.897.23-1.837.353-2.805.353-.968 0-1.908-.122-2.805-.353A17.151 17.151 0 0112 2.276zM10.122 2.43a18.629 18.629 0 00-2.37 6.49 11.266 11.266 0 01-3.746-2.504 9.754 9.754 0 016.116-3.985z"/>
</svg>
  }
}
