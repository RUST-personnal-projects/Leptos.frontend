use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use gloo_console::warn;
use wasm_bindgen::JsValue;

use super::video_details::VideoDetails;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();

    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}

#[function_component(Videos)]
pub fn video() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap() 
                    .json()
                    .await;
                let fetched_videos = match fetched_videos {
                    Ok(value) => value,
                    Err(err) => {
                        warn!(JsValue::from(err.to_string()));
                        vec![]
                    },
                };
                videos.set(fetched_videos);
            });
            || ()
        });
    }

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };
    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });    

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <VideosList videos={ (*videos).clone() } on_click={ on_video_select.clone() } />
            </div>
            { for details }
        </>
    }
    
}