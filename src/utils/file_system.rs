//! Tools for working with the file system.

use crate::error::{Error, Result};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

/// Creates a new file at the given destination.
///
/// # Arguments
///
/// * `destination` - The path to create the file at.
pub fn create_file(destination: PathBuf) -> Result<File> {
    let mut open_options = OpenOptions::new();
    open_options.read(true);
    open_options.write(true);
    open_options.create(true);

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::OpenOptionsExt;
        open_options.mode(0o755);
    }

    let file = open_options.open(destination)?;
    Ok(file)
}

/// Creates the parent directory of the given destination.
/// If the parent directory already exists, nothing is done.
///
/// # Arguments
///
/// * `destination` - The path to create the parent directory for.
pub fn create_parent_dir(destination: PathBuf) -> Result<()> {
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }

    Ok(())
}

/// Extracts a zip file to the given destination.
///
/// # Arguments
///
/// * `zip_path` - The path to the zip file.
/// * `destination` - The path to extract the zip file to.
pub fn extract_zip(zip_path: PathBuf, destination: PathBuf) -> Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let destination = destination.join(
            file.enclosed_name()
                .ok_or(Error::Unknown("Failed to get file name".to_string()))?,
        );

        match file.is_file() {
            true => {
                if let Some(parent) = destination.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                let mut dest_file = create_file(destination)?;
                std::io::copy(&mut file, &mut dest_file)?;
            }
            false => {
                std::fs::create_dir_all(destination)?;
            }
        }
    }

    Ok(())
}

/// Extracts a tar.xz file to the given destination.
///
/// # Arguments
///
/// * `tar_path` - The path to the tar.xz file.
/// * `destination` - The path to extract the tar.xz file to.
pub fn extract_tar_xz(tar_path: PathBuf, destination: PathBuf) -> Result<()> {
    let tar_gz = File::open(tar_path)?;

    let decompressor = XzDecoder::new(tar_gz);
    let mut archive = Archive::new(decompressor);

    archive.unpack(destination)?;

    Ok(())
}

/// Sets the executable bit on the given file.
///
/// # Arguments
///
/// * `executable` - The path to the executable file.
#[cfg(not(target_os = "windows"))]
pub fn set_executable(executable: PathBuf) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(executable.clone())?.permissions();

    perms.set_mode(0o755);
    std::fs::set_permissions(executable, perms)?;

    Ok(())
}
