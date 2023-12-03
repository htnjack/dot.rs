use home::home_dir;
use std::fs;
use std::os::unix;
use std::path::PathBuf;

// TODO: Split error types
// i.e. `PackageResolveError`, `FileOperationError`
#[derive(Debug, Clone)]
pub enum DotError {
    HomeNotFound,
    PackageNotFound(String),
    PackageExists(String),
    CouldNotMove(String, String, String),
    CouldNotLink(String, String, String),
}

#[derive(Debug, Clone)]
pub enum PathKind {
    Config,
    Dotfiles,
}

fn build_path(kind: PathKind, package: Option<&str>) -> Result<PathBuf, DotError> {
    let mut path = PathBuf::new();
    match home_dir() {
        Some(p) => path.push(p),
        None => {
            return Err(DotError::HomeNotFound);
        }
    };
    match kind {
        PathKind::Config => path.push(".config"),
        PathKind::Dotfiles => path.push(".dotfiles"),
    };
    if let Some(p) = package {
        path.push(p);
    }
    Ok(path)
}

fn stow(package: PathBuf, dotfile: PathBuf) -> Result<(), DotError> {
    // TODO: is there a better way? i.e. without clone?
    // no need for variables, but I left them for clarity
    let pack = package.clone();
    let dot = dotfile.clone();
    match fs::rename(pack, dot) {
        Ok(_) => {}
        Err(e) => {
            return Err(DotError::CouldNotMove(
                package.display().to_string(),
                dotfile.display().to_string(),
                e.to_string()
            ));
        }
    };

    // TODO: is there a better way? i.e. without clone?
    let pack = package.clone();
    let dot = dotfile.clone();
    match unix::fs::symlink(dot, pack) {
        Ok(_) => {},
        Err(e) => {
            return Err(DotError::CouldNotLink(
                package.display().to_string(),
                dotfile.display().to_string(),
                e.to_string()
            ));
        },
    };
    Ok(())
}

pub fn create_package(package: &str) -> Result<(), DotError> {
    let package_path = match build_path(PathKind::Config, Some(package)) {
        Ok(p) => {
            if !p.exists() {
                return Err(DotError::PackageNotFound(
                    p.display().to_string(),
                ));
            }
            if p.is_symlink() {
                return Err(DotError::PackageExists(p.display().to_string()));
            }
            p
        }
        Err(e) => {
            return Err(e);
        }
    };

    let dotfile_path = match build_path(PathKind::Dotfiles, Some(package)) {
        Ok(p) => {
            if p.exists() {
                return Err(DotError::PackageExists(p.display().to_string()));
            }
            p
        }
        Err(e) => {
            return Err(e);
        }
    };
    stow(package_path, dotfile_path)
}

pub fn list_packages() {}
