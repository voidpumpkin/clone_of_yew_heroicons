use yew::prelude::*;

#[derive(Clone, Properties, Eq, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}
