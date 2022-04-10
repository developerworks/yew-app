mod video;
mod video_details;

use reqwasm::http::Request;
pub use crate::video::{Video, VideosList};
pub use crate::video_details::{VideoDetails};

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
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
    let videos = use_state(|| vec![]);

    // 子作用域, 重用变量名? 好像是!
    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
            // 拷贝
            let videos = videos.clone();
            // 线程?
            // 这里使用了好多 unwrap(), 在实际开发中, 说是要处理错误.
            wasm_bindgen_futures::spawn_local(async move {
                let fetch_videos: Vec<Video> = Request::get("https://localhost/public/data.json")
                    .send().await.unwrap().json().await.unwrap();
                videos.set(fetch_videos);
            });
            // 这又是啥?
            || ()
        }, ());
    }

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                // <VideosList videos={videos} on_click={on_video_select.clone()} />
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()}/>
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}