use flate2::read::GzDecoder;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::model::asset_name::mk_exe_name;

pub struct Archive<'a> {
    archive_path: &'a PathBuf,
    tmp_dir: &'a Path,
    exe_name: &'a str,
    tag: &'a str,
    archive_type: ArchiveType<'a>,
}

/// Archive type that specifies how to unpack asset
enum ArchiveType<'a> {
    AppImage(&'a str),
    Exe(&'a str),
    Zip(&'a str),
    TarGz(&'a str),
}

pub enum UnpackError {
    IOError(std::io::Error),
    ZipError(zip::result::ZipError),
    ExeNotFound(String),
}

impl Display for UnpackError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            UnpackError::IOError(e) => write!(f, "{}", e),
            UnpackError::ZipError(e) => write!(f, "{}", e),
            UnpackError::ExeNotFound(archive_name) => {
                write!(f, "Can't find executable in archive: {}", archive_name)
            }
        }
    }
}

impl<'a> Archive<'a> {
    pub fn from(
        archive_path: &'a PathBuf,
        tmp_dir: &'a Path,
        exe_name: &'a str,
        asset_name: &'a str,
        tag: &'a str,
    ) -> Option<Archive<'a>> {
        match asset_name.rsplit_once('.') {
            None | Some((_, "exe")) => {
                // un-compressed binary
                return Archive {
                    archive_path,
                    tmp_dir,
                    exe_name,
                    tag,
                    archive_type: ArchiveType::Exe(asset_name),
                }
                .into();
            }
            Some((_, ext)) if ext.len() > 10 => {
                // un-compressed binary
                return Archive {
                    archive_path,
                    tmp_dir,
                    exe_name,
                    tag,
                    archive_type: ArchiveType::Exe(asset_name),
                }
                .into();
            }
            Some((_, "AppImage")) => {
                return Archive {
                    archive_path,
                    tmp_dir,
                    exe_name,
                    tag,
                    archive_type: ArchiveType::AppImage(asset_name),
                }
                .into();
            }
            Some((prefix, "tgz")) => {
                return Archive {
                    archive_path,
                    tmp_dir,
                    exe_name,
                    tag,
                    archive_type: ArchiveType::TarGz(prefix),
                }
                .into()
            }
            Some((prefix, "zip")) => {
                return Archive {
                    archive_path,
                    tmp_dir,
                    exe_name,
                    tag,
                    archive_type: ArchiveType::Zip(prefix),
                }
                .into()
            }
            _ => {
                asset_name.strip_suffix(".tar.gz").map(|tar_gz_dir| Archive {
                        archive_path,
                        tmp_dir,
                        exe_name,
                    tag,
                        archive_type: ArchiveType::TarGz(tar_gz_dir),
                    })
            }
        }
    }

    /// Unpack archive and return path to the executable tool
    pub fn unpack(&self) -> Result<PathBuf, UnpackError> {
        match self.archive_type {
            // already .AppImage file: no need to unpack
            ArchiveType::AppImage(app_image) => Ok(self.tmp_dir.join(app_image)),

            // already .exe file without archive (on Windows): no need to unpack
            ArchiveType::Exe(exe_file) => Ok(self.tmp_dir.join(exe_file)),

            // unpack .tar.gz archive
            ArchiveType::TarGz(asset_name) => {
                unpack_tar(self.archive_path, self.tmp_dir).map_err(UnpackError::IOError)?;
                find_path_to_exe(self.archive_path, self.tmp_dir, self.exe_name, asset_name, self.tag)
            }

            // unpack .zip archive
            ArchiveType::Zip(asset_name) => {
                unpack_zip(self.archive_path, self.tmp_dir)?;
                find_path_to_exe(self.archive_path, self.tmp_dir, self.exe_name, asset_name, self.tag)
            }
        }
    }
}

fn unpack_tar(
    tar_path: &PathBuf,
    tmp_dir: &Path,
) -> Result<(), std::io::Error> {
    // unpack tar_path to tmp_dir
    let tar_file = File::open(tar_path)?;
    let tar_decoder = GzDecoder::new(tar_file);
    let mut archive = tar::Archive::new(tar_decoder);
    archive.unpack(tmp_dir)
}

fn unpack_zip(
    zip_path: &PathBuf,
    tmp_dir: &Path,
) -> Result<(), UnpackError> {
    let zip_archive_file = File::open(zip_path).map_err(UnpackError::IOError)?;

    let mut archive = zip::ZipArchive::new(zip_archive_file).map_err(UnpackError::ZipError)?;

    archive.extract(tmp_dir).map_err(UnpackError::ZipError)
}

fn find_path_to_exe(
    archive_path: &Path,
    tmp_dir: &Path,
    exe_name: &str,
    asset_name: &str,
    tag: &str,
) -> Result<PathBuf, UnpackError> {
    let path_candidates = exe_paths(exe_name, asset_name, tag);

    // find a path
    for path in path_candidates {
        // create path to the final executable
        let mut tool_path = PathBuf::new();
        tool_path.push(tmp_dir);
        tool_path.push(path);

        // check if this path actually exists
        if tool_path.is_file() {
            return Ok(tool_path);
        }
    }

    Err(UnpackError::ExeNotFound(format!(
        "{}",
        archive_path.display()
    )))
}

// List of potential paths where an executable can be inside the archive
fn exe_paths(
    exe_name: &str,
    asset_name: &str,
    tag: &str,
) -> Vec<PathBuf> {
    let exe_name = mk_exe_name(exe_name);

    vec![
        asset_name.into(),
        asset_name.trim_end_matches(".tar.gz").into(),
        asset_name.trim_end_matches(".tgz").into(),
        asset_name.trim_end_matches(".zip").into(),

        exe_name.clone().into(),
        [asset_name, &exe_name].iter().collect(),
        ["tmp", asset_name, &exe_name].iter().collect(),
        [&exe_name, &exe_name].iter().collect(),
        ["bin", &exe_name].iter().collect(),
        [asset_name, "bin", &exe_name].iter().collect(),

        [&format!("{exe_name}-{tag}"), &exe_name].iter().collect(),
        [&format!("{exe_name}-{}", tag.trim_start_matches('v')), &exe_name].iter().collect(),
    ]
}
