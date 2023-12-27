use crate::utils::commands::programs::{open_repo::OpenRepoComponent, welcome::WelcomeComponent};

use super::{
    help::{HelpComponent, HelpProps},
    legacy::{LegacyComponent, LegacyProps},
    open_repo::OpenRepoProps,
    welcome::WelcomeProps,
};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum OutputComponent {
    Help(HelpProps),
    Legacy(LegacyProps), // this is for compatibility. will be removed in the future
    Welcome(WelcomeProps),
    OpenRepo(OpenRepoProps),
}

pub fn render_child(child: &Box<OutputComponent>) -> Html {
    match **child {
        OutputComponent::Help(ref props) => html! { <HelpComponent args={props.args.clone()}  /> },
        OutputComponent::Legacy(ref props) => {
            html! { <LegacyComponent legacy_output={props.legacy_output.clone()} /> }
        }
        OutputComponent::Welcome(ref _props) => {
            html! { <WelcomeComponent /> }
        }
        OutputComponent::OpenRepo(ref props) => {
            html! { <OpenRepoComponent config={props.config} /> }
        }
    }
}
