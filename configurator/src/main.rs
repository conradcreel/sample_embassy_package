use std::io::Write;
use std::{
    fs::{self, File},
    path::Path,
};

use anyhow::Context;
use http::Uri;
use serde::{
    de::{Deserializer, Error as DeserializeError, Unexpected},
    Deserialize,
};

fn deserialize_parse<'de, D: Deserializer<'de>, T: std::str::FromStr>(
    deserializer: D,
) -> Result<T, D::Error> {
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse()
        .map_err(|_| DeserializeError::invalid_value(Unexpected::Str(&s), &"a valid URI"))
}

fn parse_quick_connect_url(url: Uri) -> Result<(String, String, String, u16), anyhow::Error> {
    let auth = url
        .authority()
        .ok_or_else(|| anyhow::anyhow!("invalid Quick Connect URL"))?;
    let mut auth_split = auth.as_str().split(|c| c == ':' || c == '@');
    let user = auth_split
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing user"))?;
    let pass = auth_split
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing pass"))?;
    let host = url.host().unwrap();
    let port = url.port_u16().unwrap_or(8332);
    Ok((user.to_owned(), pass.to_owned(), host.to_owned(), port))
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    tor_address: String
}

#[derive(serde::Serialize)]
pub struct Property<T> {
    #[serde(rename = "type")]
    value_type: &'static str,
    value: T,
    description: Option<String>,
    copyable: bool,
    qr: bool,
    masked: bool,
}

fn main() -> Result<(), anyhow::Error> {
    fs::create_dir_all("/datadir/mvctest/Main/")?;
    
    /*
    let config: Config = serde_yaml::from_reader(
        File::open("/datadir/start9/config.yaml").with_context(|| "/datadir/start9/config.yaml")?,
    )?;
    let tor_address = config.tor_address;

    let addr = tor_address.split('.').collect::<Vec<&str>>();
    match addr.first() {
        Some(x) => {
            print!("{}", format!("export MVCTEST_HOST='https://{}.local/'\n", x));
        }
        None => {}
    }
    */

    // write backup ignore to the root of the mounted volume
    std::fs::write(
        Path::new("/datadir/.backupignore.tmp"),
        include_str!("./templates/.backupignore.template"),
    )?;
    std::fs::rename("/datadir/.backupignore.tmp", "/datadir/.backupignore")?;

    Ok(())
}
