use yew::prelude::*;
use crate::props::Props;

#[function_component]
pub fn ArrowTrendingUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {{class}} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.941"/>
</svg>
  }
}