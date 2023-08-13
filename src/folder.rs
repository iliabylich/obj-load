use std::io::{Read, Write};

use walkdir::WalkDir;

pub fn zip(relative_path: &str) -> String {
    let dir_name = std::path::Path::new(relative_path)
        .file_name()
        .expect("Invalid filepath given")
        .to_str()
        .expect("Invalid filepath given")
        .to_string();
    let out_zip_file_name = format!("{}.zip", dir_name);
    let zip_file = std::fs::File::create(&out_zip_file_name).unwrap();

    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let zip_options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let mut buffer = Vec::new();
    for entry in WalkDir::new(relative_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let name = path
            .strip_prefix(std::path::Path::new(relative_path))
            .unwrap();

        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip_writer.start_file_from_path(name, zip_options).unwrap();
            let mut f = std::fs::File::open(path).unwrap();

            f.read_to_end(&mut buffer).unwrap();
            zip_writer.write_all(&buffer).unwrap();
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip_writer
                .add_directory_from_path(name, zip_options)
                .unwrap();
        }
    }

    out_zip_file_name
}

pub fn unzip(relative_path: &str) -> String {
    let target_dir = relative_path.strip_suffix(".zip").unwrap();
    let file = std::fs::File::open(relative_path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        outpath = std::path::PathBuf::from(target_dir).join(outpath);

        if (*file.name()).ends_with('/') {
            println!("Extracting \"{}\"", outpath.display());
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "Extracting \"{}\" ({} bytes)",
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    std::fs::remove_file(relative_path).unwrap();

    target_dir.to_string()
}
