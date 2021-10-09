use std::fs::File;

use zip::ZipArchive;

pub fn open_package() -> std::io::Result<ZipArchive<File>> {
    let file = File::open(crate::recovery::PKG_FILE.lock().unwrap().as_ref().unwrap())?;
    let archive = ZipArchive::new(file)?;
    Ok(archive)
}

pub fn extract_file(archive: &mut ZipArchive<File>, path: &str, dest: &str) -> std::io::Result<()> {
    let mut entry = archive.by_name(path)?;
    let mut dest = File::create(dest)?;
    let bytes_copied = std::io::copy(&mut entry, &mut dest)?;
    assert_eq!(bytes_copied, entry.size());
    Ok(())
}
