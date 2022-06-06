use crate::components::history::history_component::HistoryComponent;
use crate::components::history::hook::use_history;
use crate::components::history::input::Input;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainProps {
    input_ref: NodeRef,
}

#[function_component(Main)]
pub fn main(props: &MainProps) -> Html {
    let history_props = use_history();
    let container_ref = use_node_ref();
    let index = *(history_props.last_command_index.clone());

    //   const init = React.useCallback(() => setHistory(banner()), []);

    //   React.useEffect(() => {
    //     init();
    //   }, [init]);

    //   React.useEffect(() => {
    //     if (inputRef.current) {
    //       inputRef.current.scrollIntoView();
    //       inputRef.current.focus({ preventScroll: true });
    //     }
    //   }, [history]);

    html! {
        <>
      <div className="p-8 overflow-hidden h-full border-2 rounded border-light-yellow dark:border-dark-yellow">
        <div ref={container_ref} className="overflow-y-auto h-full">
          <HistoryComponent />
          <Input
            input_ref={&props.input_ref.clone()}
            container_ref={container_ref}
          />
        </div>
      </div>
    </>
    }
}

// import Head from 'next/head';
// import React from 'react';
// import config from '../../config.json';
// import { Input } from '../components/input';
// import { useHistory } from '../components/history/hook';
// import { History } from '../components/history/History';
// import { banner } from '../utils/bin';

// interface IndexPageProps {
//   inputRef: React.MutableRefObject<HTMLInputElement>;
// }

// const IndexPage: React.FC<IndexPageProps> = ({ inputRef }) => {
//   const containerRef = React.useRef(null);
//   const {
//     history,
//     command,
//     lastCommandIndex,
//     setCommand,
//     setHistory,
//     clearHistory,
//     setLastCommandIndex,
//   } = useHistory([]);

//   const init = React.useCallback(() => setHistory(banner()), []);

//   React.useEffect(() => {
//     init();
//   }, [init]);

//   React.useEffect(() => {
//     if (inputRef.current) {
//       inputRef.current.scrollIntoView();
//       inputRef.current.focus({ preventScroll: true });
//     }
//   }, [history]);

//   return (
//     <>
//       <Head>
//         <title>{config.title}</title>
//       </Head>

//       <div className="p-8 overflow-hidden h-full border-2 rounded border-light-yellow dark:border-dark-yellow">
//         <div ref={containerRef} className="overflow-y-auto h-full">
//           <History history={history} />

//           <Input
//             inputRef={inputRef}
//             containerRef={containerRef}
//             command={command}
//             history={history}
//             lastCommandIndex={lastCommandIndex}
//             setCommand={setCommand}
//             setHistory={setHistory}
//             setLastCommandIndex={setLastCommandIndex}
//             clearHistory={clearHistory}
//           />
//         </div>
//       </div>
//     </>
//   );
// };
