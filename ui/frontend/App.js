import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

import {
    getAvailableTasks, getPoints, nftToken,
    TestCode
} from './near-api';
import {SignInPrompt, SignOutButton} from './ui-components';

import Editor from 'react-simple-code-editor';
import {highlight, languages} from 'prismjs/components/prism-core';
import 'prismjs/components/prism-clike';
import 'prismjs/components/prism-javascript';
import 'prismjs/themes/prism.css'; //Example style, you can use another

const codeHeaderTemplate = `/*
 * Complete the 'getResult' function below and return the result.
 */
 
 function getResult(%PARAMS%) {
    // Write your code here
 `;

export default function App() {
    const [firstDataLoaded, setFirstDataLoaded] = React.useState(false);
    const [availableTasks, setAvailableTasks] = React.useState([]);
    const [points, setPoints] = React.useState(0);
    const [badge, setBadge] = React.useState("");
    const [selectedTask, setSelectedTask] = React.useState(null);
    const [code, setCode] = React.useState(``);
    const [codeHeader, setCodeHeader] = React.useState("");
    const [result, setResult] = React.useState("");
    const [wrongTests, setWrongTests] = React.useState(0);
    const [tasks, setTasks] = React.useState([]);


    const [uiPleaseWait, setUiPleaseWait] = React.useState(true);

    // Get blockchian state once on component load
    React.useEffect(() => {
        if (window.walletConnection.isSignedIn()) {
            LoadTasks();
        }
    }, []);

    /// If user not signed-in with wallet - show prompt
    if (!window.walletConnection.isSignedIn()) {
        // Sign-in flow will reload the page later
        return <SignInPrompt/>;
    }

    function LoadTasks() {
        getAvailableTasks(window.accountId)
            .then(availableTasks => {
                setAvailableTasks(availableTasks);

                // Group by Category
                let _tasks = [];
                let tasks_by_categories = [];
                availableTasks.map(task => {
                    if (!tasks_by_categories.hasOwnProperty(task.category)) {
                        tasks_by_categories[task.category] = [];
                    }
                    tasks_by_categories[task.category].push(task);
                });

                Object.keys(tasks_by_categories).map(category => {
                    _tasks.push(<h3 key={category}>{category}</h3>);
                    tasks_by_categories[category].map(task => {
                            _tasks.push(<li key={task.task_id}><a style={{cursor: 'pointer'}} onClick={_ => {
                                loadTask(task);
                                return false
                            }}> {task.title.toString()} </a></li>)
                        }
                    )
                });

                setTasks(_tasks);
            })
            .then(_ => getPoints(window.accountId))
            .then(points => {
                setPoints(points);
                if (points > 0) {
                    nftToken(window.accountId)
                        .then(token => {
                            if (token.hasOwnProperty("metadata") && token.metadata.hasOwnProperty("media")) {
                                let svg = decodeURIComponent((token.metadata.media.substr("data:image/svg xml,".length) +'').replace(/\+/g, '%20'));
                                svg = svg.replace("<svg ", "<svg width='300' height='300' ");
                                setBadge(svg);
                            }
                        })
                }
            })
            .then(_ => setFirstDataLoaded(true))
            .catch(alert)
            .finally(() => {
                setUiPleaseWait(false);
            });
    }

    function proceed(e) {
        setWrongTests(0);
        setResult("")
        setSelectedTask(null);
        setCodeHeader("");
        LoadTasks();
    }

    function submitCode(e) {
        e.preventDefault();
        setUiPleaseWait(true);
        const {codeEditor} = e.target.elements;
        TestCode(selectedTask.task_id, codeEditor.value)
            .then(res => {
                if (res && res.length > 1) {
                    let all_tests = res[0];
                    let wrong_tests = res[1];
                    let result = (wrong_tests === 0);

                    setResult(result
                        ? "Congratulations! You solved this challenge. Would you like to challenge your friends?"
                        : `${all_tests}/${wrong_tests} test cases failed :(`
                    );
                    setWrongTests(wrong_tests);
                } else {
                    alert("Network error. Please check the dev console.");
                }
            })
            .catch(alert)
            .finally(() => {
                setUiPleaseWait(false);
            });
    }

    function ExitTaskButton({accountId}) {
        return (
            <button style={{float: 'right'}} onClick={_ => {
                proceed()
            }}>
                Exit Challenge
            </button>
        );
    }


    function getTask(task_id) {
        let tasks = availableTasks.filter(task => task.task_id === task_id);

        if (tasks.length) {
            console.log(tasks);
            setCodeHeader(codeHeaderTemplate.replace("%PARAMS%", tasks[0].parameters));
            return tasks[0];
        } else {
            return {};
        }
    }

    function loadTask(task) {
        setCodeHeader(codeHeaderTemplate.replace("%PARAMS%", task.parameters));
        setSelectedTask(task);

        setTimeout(() => {
            let editors = document.getElementsByClassName("npm__react-simple-code-editor__textarea");
            if (editors.length > 1) {
                document.getElementsByClassName("npm__react-simple-code-editor__textarea")[1].focus();
            }
        }, 0);
    }




    return (
        <>
            <main className={uiPleaseWait ? 'please-wait' : ''}>

                {(selectedTask == null || selectedTask.task_id == -1) &&
                    <div className={"menu-container"}>
                        <div className={"sign-out"}>
                            <SignOutButton accountId={window.accountId}/>
                        </div>

                        {firstDataLoaded &&
                            <>
                                <h1>
                                    Web3 JsRank
                                </h1>
                                <h5 style={{textAlign: "center"}}>
                                    Available challenges: {Object.keys(availableTasks).length}
                                </h5>
                                {tasks.length === 0 &&
                                    <div style={{textAlign: "center"}}>Come back, we will add new challenges soon</div>
                                }

                                <ul>
                                    {tasks.map(task => task)}
                                </ul>

                                {points > 0 && <>
                                    <hr/>
                                    <h2>Challenges solved: {points}</h2>

                                    {badge && <div style={{textAlign: "center"}}>
                                        <p>Your JsRank NFT:</p>
                                        <div dangerouslySetInnerHTML={{__html: badge}} />


                                    </div>
                                    }
                                </>
                                }
                            </>}

                        {!firstDataLoaded &&
                            <h1>Loading...</h1>
                        }

                    </div>
                }


                {selectedTask != null && selectedTask.task_id >= 0 &&
                    <div className={"container"}>
                        <div className={"challenge-info"}>


                            <h2>{selectedTask.title}</h2>
                            <div dangerouslySetInnerHTML={{__html: selectedTask.description}}></div>


                        </div>
                        <div className={"challenge-submit"}>

                            <div className={"sign-out"}>
                                <ExitTaskButton/>
                            </div>

                            <div className={"challenge-submit-form"} style={{paddingLeft: "10px"}}>

                                <form onSubmit={submitCode} className="change">

                                    <Editor
                                        disabled
                                        value={codeHeader}
                                        onValueChange={code => setCodeHeader(code)}
                                        highlight={code => highlight(code, languages.js)}
                                        padding={0}
                                        style={{
                                            fontFamily: '"Fira code", "Fira Mono", monospace',
                                            fontSize: 16,

                                        }}
                                    />

                                    <div style={{paddingLeft: "30px"}}>
                                        <Editor
                                            name={"codeEditor"}
                                            value={code}
                                            onValueChange={code => setCode(code)}
                                            highlight={code => highlight(code, languages.js)}
                                            padding={10}
                                            style={{
                                                fontFamily: '"Fira code", "Fira Mono", monospace',
                                                fontSize: 16,

                                            }}
                                        />
                                    </div>

                                    <Editor
                                        disabled
                                        value="}"
                                        highlight={code => highlight(code, languages.js)}
                                        padding={0}
                                        style={{
                                            fontFamily: '"Fira code", "Fira Mono", monospace',
                                            fontSize: 16,

                                        }}
                                    />


                                    {result && <div className="result">

                                        <div
                                            className={`result ${(wrongTests === 0 ? "success" : "failed")}`}>{result}</div>

                                        {wrongTests === 0 && <div className={"proceed"}>
                                            <form onSubmit={proceed} className="change">
                                                <button>
                                                    <span>Proceed</span>
                                                </button>
                                            </form>
                                        </div>
                                        }
                                    </div>
                                    }

                                    {(!result || wrongTests > 0) &&
                                        <div>
                                            <button>
                                                <span>Submit code</span>
                                                <div className="loader"></div>
                                            </button>
                                        </div>
                                    }

                                </form>
                            </div>
                        </div>
                    </div>
                }


            </main>
        </>
    );
}
