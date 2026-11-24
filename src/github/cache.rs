use std::{env, path::PathBuf};

pub(super) fn root() -> Option<PathBuf> {
    let override_path = env::var_os("RKAN_CACHE_HOME").filter(|path| !path.is_empty());
    if let Some(path) = override_path {
        return Some(path.into());
    }

    let xdg_cache_home = env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute());

    xdg_cache_home
        .or_else(|| env::home_dir().map(|home| home.join(".cache")))
        .map(|path| path.join("rkan"))
}

pub(super) fn asset_sha256(digest: &str) -> Option<String> {
    let sha256 = digest.strip_prefix("sha256:")?;
    (sha256.len() == 64 && sha256.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| sha256.to_ascii_uppercase())
}
