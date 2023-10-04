extern crate base64;
use reqwasm::http::Request;
use gloo_console::log;
use yew::prelude::*;
use stylist::{css, Style};
const STYLE: &str = include_str!("style.css");
const TOKEN: &str = "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";
#[derive(PartialEq, Properties)]
pub struct TagProps {
    name: TagOption,
    a: UseStateHandle<usizi>
}


#[derive(PartialEq)]
pub enum TagOption{
    Star,
    Pr,
    Commit
}


impl std::fmt::Display for TagOption{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self{
            Self::Star => "star",
            Self::Pr => "pr",
            Self::Commit => "commit"
        };
        write!(f, "{name}")
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GithubLastCommit{
    html_url: String,
    name: String
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubRepo{
    stargazers_count: usize,
    name: String
}


#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubRepoCommit{
    sha: String
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
pub struct GithubPr{
    total_count: usize
}


#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubCommit{
    items: Vec<String>
}
1
#[function_component]
pub fn Tag(props: &TagProps) -> Html {
    let TagProps { name, a } = props;
    let src = format!("img/{name}.svg");
    // let a = format!("{}", TOKEN);
    let call = match &name{
        TagOption::Star => {
            let a: UseStateHandle<usize> = a.clone();
            Callback::from(move |_|{
                let a = a.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut n = 0;
                    log!("start");
                    let res = Request::get("https://api.github.com/users/5-23/repos?per_page=1000")
                        .header("Authorization", &format!("token {TOKEN}"))
                        .send().await.unwrap()
                        .json::<Vec<GithubRepo>>().await.unwrap();
                    for i in res{
                        n += i.stargazers_count
                    }
                    a.set(n);
                    log!(n.to_string());
                });      
            })
        },
        TagOption::Pr => {
            let a = a.clone();
            Callback::from(move |_|{
                let a = a.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut n = 0;
                    log!("start");
                    let res = Request::get("https://api.github.com/search/issues?q=is:pr+author:5-23")
                        .header("Authorization", &format!("token {TOKEN}"))
                        .send().await.unwrap()
                        .json::<GithubPr>().await.unwrap();
                    n += res.total_count;
                    a.set(n);
                    log!(n.to_string());
                });      
            })
        },
        TagOption::Commit => {
            let a = a.clone();
            Callback::from(move |_|{
                let a = a.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut n = 0;
                    let mut big = *a;
                    log!(big.to_string());
                    log!("start");
                    let res = Request::get("https://api.github.com/users/5-23/repos?per_page=1000")
                        .header("Authorization", &format!("token {TOKEN}"))
                        .send().await.unwrap()
                        .json::<Vec<GithubRepo>>().await.unwrap();
                    for i in res{
                        let c = Request::get(&format!("https://api.github.com/repos/5-23/{}/commits", i.name))
                            .header("Authorization", &format!("token {TOKEN}"))
                            .send().await.unwrap()
                            .json::<Vec<GithubRepoCommit>>().await.unwrap_or(vec![]);
                        n += c.len();
                        if n > big{
                            big = n;
                        }
                    }
                    a.set(n);
                    log!(n.to_string());
                });      
            })
        }
    };
    gloo::timers::callback::Timeout::new(1_000, move|| {
            let a = Event::new("a").unwrap();
            call.emit(a);
    }).forget();
    // let n = async {
    //     let mut n = 0;
    //     let res = Request::patch("https://api.github.com/users/5-23/repos")
    //         .header("Authorization", &format!("token {TOKEN}"))
    //         .send().await.unwrap()
    //         .json::<Vec<GithubRepo>>().await.unwrap();
    //     for i in res{
    //         n += i.stargazers_count;
    //     }
    //     a.set(n);
    //     return n
    // };
    
    html! {
        <div class={css!{
            background-color: #EFF1F5;
            color: #7287FD;
            display: inline-flex;
            gap: 5px;
            padding: 8px 16px;
            border-radius: 8px;
        }}>
            <img src={src} alt="star"/>
            <b>{if **a == 0{"loading...".to_string()}else{(**a).to_string()}}</b>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let style = Style::new(STYLE).unwrap();
    let state1 = use_state(||0);
    let state2 = use_state(||0);
    let state3 = use_state(||0);
    wasm_bindgen_futures::spawn_local(async move {
        let res = Request::get("https://api.github.com/user/repos?sort=updated&per_page=1&page=1")
            .header("Authorization", &format!("token {TOKEN}"))
            .send().await.unwrap()
            .json::<Vec<GithubLastCommit>>().await.unwrap();
        log!(serde_json::to_string_pretty(&res).unwrap());
    });

    html! {
    <body class={style}>
        <main>
            <h1>{"Red Sus"}</h1>
            <article>
                <Tag name={TagOption::Star} a={state1}/>
                <Tag name={TagOption::Pr} a={state2}/>
                <Tag name={TagOption::Commit} a={state3}/>
            </article>
        </main>
    </body>
    }
} 
fn main() {
    yew::Renderer::<App>::new().render();
}