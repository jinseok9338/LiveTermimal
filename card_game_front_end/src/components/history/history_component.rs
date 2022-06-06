use crate::components::{history::hook::use_history, ps_1::Ps1};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryComponentProps {}

#[function_component(HistoryComponent)]
pub fn history_compoenet(props: &HistoryComponentProps) -> Html {
    let history = use_history();
    let history_cloned = &*(history.clone().history);

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
