use std::str::FromStr;

#[cfg(target_arch = "wasm32")]
use js_sys::{ArrayBuffer, Uint8Array};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::FileReader;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, Response};

pub(crate) async fn load_data(filename: &str) -> anyhow::Result<Vec<u8>> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let uri = format!(
            "{}/assets/{}",
            window.location().origin().unwrap(),
            filename
        );

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(web_sys::RequestMode::Cors);

        let request = Request::new_with_str_and_init(uri.as_str(), &opts).unwrap();
        request.headers().set("Accept", "image/png").unwrap();
        request.headers().set("Content-Type", "image/png").unwrap();

        let value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();
        assert!(value.is_instance_of::<Response>());
        let resp: Response = value.dyn_into().unwrap();
        let data = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        if let Some(buf) = data.dyn_ref::<ArrayBuffer>() {
            let buf = Uint8Array::new(buf);
            Ok(buf.to_vec())
        } else {
            Ok(vec![])
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = std::path::PathBuf::from_str(file!()).unwrap();
        let exe_path = path.parent().unwrap().to_str().unwrap();
        let path = format!("{}/../assets/{}", exe_path, filename);
        println!("{}", path);
        Ok(std::fs::read(path)?)
    }
}
