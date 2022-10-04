use yew::prelude::*;
use crate::props::Props;

#[function_component]
pub fn PlusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {{class}} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"/>
</svg>
  }
}