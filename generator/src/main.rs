mod article;
mod page;
mod templates;
mod site;
mod builder;

use site::Site;
use std::env;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let base_url = args[1].to_string();
    let root_path = args[2].to_string();
    let output_path = args[3].to_string();

    let site = Site::from_root_path(&base_url, &root_path);
    builder::build_site(site, &root_path, &output_path)?;
    Ok(())
}

