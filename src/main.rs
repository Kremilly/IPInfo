mod core;

use std::{
    env,
    error::Error,
};

use crate::core::ipinfo::IPInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    IPInfo::get_info(args).await?;

    Ok(())
}
