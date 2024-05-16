use std::fs;
use std::io::Read;
use serde_json::from_str;
use zip::ZipArchive;
use crate::OuterManifest;

pub fn read_zip_file(path: &str) -> crate::Result<(Vec<u8>, Vec<u8>)> {
    let reader = fs::File::open(path)?;
    let mut archive = ZipArchive::new(reader)?;
    let application = {
        let mut file = archive.by_name("manifest.json")?;
        let mut manifest_string = String::new();
        file.read_to_string(&mut manifest_string)?;
        let outer = from_str::<OuterManifest>(&manifest_string)?;
        outer.manifest.application
    };
    let dat_file = {
        let mut file = archive.by_name(&application.dat_file)?;
        let mut dat_vec = Vec::new();
        file.read_to_end(&mut dat_vec)?;
        dat_vec
    };
    let bin_file = {
        let mut file = archive.by_name(&application.bin_file)?;
        let mut bin_vec = Vec::new();
        file.read_to_end(&mut bin_vec)?;
        bin_vec
    };
    Ok((dat_file, bin_file))
}