use super::interface::History;
use crate::components::ps_1::Ps1;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryComponentProps {
    history: Vec<History>,
}

#[function_component(HistoryComponent)]
pub fn history_compoenet(props: &HistoryComponentProps) -> Html {
    let history_cloned = props.history.clone();

    html! {
        <>
            {history_cloned.into_iter().map(|history| {
            html!{ <div key={history.clone().to_owned().command}>
                      <div class="flex flex-row space-x-2">
                        <div class="flex-shrink">
                          <Ps1 />
                        </div>

                        <div class="flex-grow">{history.clone().to_owned().command}</div>
                      </div>

                      <p
                        class="whitespace-pre-wrap mb-2"
                        // dangerouslySetInnerHTML={{ __html: history.output }}
                      />
                    </div>}
            }).collect::<Html>()}
        </>
    }
}
