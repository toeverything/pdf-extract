use rust_embed::Embed;
use std::{cell::OnceCell, collections::HashMap};

#[derive(Embed)]
#[folder = "assets/cmap/Adobe-CNS1-7/CMap/"]
struct CNS;

#[derive(Embed)]
#[folder = "assets/cmap/Adobe-GB1-6/CMap/"]
struct GB;

#[derive(Embed)]
#[folder = "assets/cmap/Adobe-Identity-0/CMap/"]
struct Identity;

#[derive(Embed)]
#[folder = "assets/cmap/Adobe-Japan1-7/CMap/"]
struct Japan;

#[derive(Embed)]
#[folder = "assets/cmap/Adobe-Korea1-2/CMap/"]
struct Korea;

#[derive(Embed)]
#[folder = "assets/cmap/Adobe-KR-9/CMap/"]
struct KR;

#[derive(Embed)]
#[folder = "assets/cmap/Adobe-Manga1-0/CMap/"]
struct Manga;

const CMAPS: OnceCell<HashMap<String, Vec<u8>>> = OnceCell::new();

pub fn get_cmap(name: &str) -> Option<Vec<u8>> {
    CMAPS
        .get_or_init(|| {
            let mut map = HashMap::new();

            for name in CNS::iter() {
                if let Some(buffer) = CNS::get(&name) {
                    map.insert(name.to_string(), buffer.data.to_vec());
                }
            }
            for name in GB::iter() {
                if let Some(buffer) = GB::get(&name) {
                    map.insert(name.to_string(), buffer.data.to_vec());
                }
            }
            for name in Identity::iter() {
                if let Some(buffer) = Identity::get(&name) {
                    map.insert(name.to_string(), buffer.data.to_vec());
                }
            }
            for name in Japan::iter() {
                if let Some(buffer) = Japan::get(&name) {
                    map.insert(name.to_string(), buffer.data.to_vec());
                }
            }
            for name in Korea::iter() {
                if let Some(buffer) = Korea::get(&name) {
                    map.insert(name.to_string(), buffer.data.to_vec());
                }
            }
            for name in KR::iter() {
                if let Some(buffer) = KR::get(&name) {
                    map.insert(name.to_string(), buffer.data.to_vec());
                }
            }
            for name in Manga::iter() {
                if let Some(buffer) = Manga::get(&name) {
                    map.insert(name.to_string(), buffer.data.to_vec());
                }
            }

            map
        })
        .get(name)
        .cloned()
}
