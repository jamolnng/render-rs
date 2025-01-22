#[cfg(target_arch = "wasm32")]
fn format_url(file_name: &str) -> reqwest::Url {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let base = reqwest::Url::parse(&format!(
        "{}/{}/",
        location.origin().unwrap(),
        option_env!("RES_PATH").unwrap_or("assets"),
    ))
    .unwrap();
    base.join(file_name).unwrap()
}

pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let url = format_url(file_name);
        let txt = reqwest::get(url).await?.text().await?;
        Ok(txt)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = std::env::current_dir().unwrap().join("assets").join(file_name);
        println!("{:?}", path);
        let txt = std::fs::read_to_string(path)?;

        Ok(txt)
    }
}

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    #[cfg(target_arch = "wasm32")]
    {
        let url = format_url(file_name);
        let data = reqwest::get(url).await?.bytes().await?.to_vec();

        Ok(data)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = std::env::current_dir().unwrap().join("assets").join(file_name);
        let data = std::fs::read(path)?;

        Ok(data)
    }
}
