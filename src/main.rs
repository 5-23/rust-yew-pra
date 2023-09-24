use stylist::css;
use yew::prelude::*;
use stylist::Style;


#[derive(PartialEq, Clone, Properties)]
struct Video {
    id: usize,
    title: String,
}

#[derive(PartialEq, Properties)]
pub struct VideoListProps {
    video: Video,
}

#[function_component(VideoHtml)]
pub fn video(VideoListProps { video }: &VideoListProps) -> Html {
    let text = use_state(||"click here!".to_string());

    let v = video.title.clone();

    let onclick = {
        let text = text.clone();
        
        move |_| {
            text.set(format!("{v}"));
        }
    };


    html! {
        <>
            <div {onclick}>
            <h1 class={if *text == "click here!".to_string(){ css!(r#"background-color: #ddd;"#) }else{css!()}}>{(*text).clone()}</h1>   <h2>{video.id.to_string()}</h2>  </div>
        </>
    }
}

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
            <VideoHtml video={video}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
