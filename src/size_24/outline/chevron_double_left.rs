use yew::prelude::*;
use crate::props::Props;

#[function_component]
pub fn ChevronDoubleLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {{class}} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M18.75 19.5l-7.5-7.5 7.5-7.5m-6 15L5.25 12l7.5-7.5"/>
</svg>
  }
}