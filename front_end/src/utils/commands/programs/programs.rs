use crate::utils::commands::programs::welcome::WelcomeComponent;

use super::{
    help::{HelpComponent, HelpProps},
    legacy::{LegacyComponent, LegacyProps},
    welcome::WelcomeProps,
};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum OutputComponent {
    Help(HelpProps),
    Legacy(LegacyProps),
    Welcome(WelcomeProps),
}

pub fn render_child(child: &Box<OutputComponent>) -> Html {
    match **child {
        OutputComponent::Help(ref props) => html! { <HelpComponent args={props.args.clone()}  /> },
        OutputComponent::Legacy(ref props) => {
            html! { <LegacyComponent legacy_output={props.legacy_output.clone()} /> }
        }
        OutputComponent::Welcome(ref props) => {
            html! { <WelcomeComponent /> }
        }
    }
}
