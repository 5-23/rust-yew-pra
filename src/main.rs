use yew::prelude::*;

struct Video {
    id: usize,
    title: String,
}

#[function_component]
fn app() -> Html {
    let videos = vec![Video{ id: 1, title: "sus".to_string() }, Video{ id: 2, title: "sussy baka".to_string() }];
    // let videos = ;
    html! {
        <>
            <h1>{"Hello, world!"}</h1>
            {videos.iter().map(|v| {html!{ <h2 id={v.id.to_string()}>{&v.title}</h2> }}).collect::<Html>()}
        </>
    }
}

fn main() {
    yew::Renderer::<app>::new().render();
}
