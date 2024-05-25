use std::{
    fs,
    error::Error,

    path::{
        Path,
        PathBuf,
    },
};

pub struct Download;

impl Download {

    pub fn extract_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
        let dir_path = Path::new(path);

        let parent_dir = match dir_path.parent() {
            Some(dir) => dir,
            None => return Err("O caminho não possui um diretório pai".into()),
        };

        let file_path = parent_dir.join("data");

        if !file_path.exists() {
            fs::create_dir_all(&file_path)?;
        }

        Ok(file_path.join("servers.json"))
    }

    async fn content(url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
    }

    pub async fn download(path: &str) -> Result<PathBuf, Box<dyn Error>> {
        let url = "https://gist.githubusercontent.com/kremilly/7b146d846ffe4f97c41feb7df24384a4/raw/503657d9320677332b69972f029aad385cad2521/servers.json";

        let path = Self::extract_path(path)?;
        let contents = Self::content(url).await?;

        fs::write(&path, contents)?;
        Ok(path)
    }

}
