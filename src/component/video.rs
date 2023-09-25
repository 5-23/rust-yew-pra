use stylist::css;
use yew::prelude::*;


#[derive(PartialEq, Clone, Properties)]
pub struct Video {
    pub id: usize,
    pub title: String,
}

#[derive(PartialEq, Properties)]
pub struct VideoListProps {
    pub video: Video,
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