mod component;

use component::video::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let video = 
        Video {
            id: 1,
            title: "hmm".to_string(),
        };
    // let videos = ;
    html! {
        <>
            <VideoHtml video={video.clone()}/>
            <VideoHtml video={video}/>
        </>
    }
} 
fn main() {
    yew::Renderer::<App>::new().render();
}