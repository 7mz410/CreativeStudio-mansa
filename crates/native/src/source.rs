//! Where release information comes from.
//!
//! [`GitHubSource`] asks the GitHub REST API only (one request per app, revalidated with ETags).
//! When the API is out of quota or unreachable, the last saved answer is shown instead.

use std::collections::BTreeMap;
use std::sync::{Mutex, PoisonError};

use artcraft_launcher_core::protocol::{LatestRelease, Source};
use artcraft_launcher_core::release::{self, Release};
use serde::{Deserialize, Serialize};

use crate::http::{Fetched, Http};

/// One answer from a source.
#[derive(Clone, Debug)]
pub struct Answer {
    pub latest: LatestRelease,
    pub source: Source,
}

/// Looks up latest releases. Called from worker threads.
pub trait ReleaseSource: Send + Sync {
    fn latest(&self, repo: &str, prereleases: bool) -> Answer;
    /// The last answer saved for `repo`, for an instant first paint.
    fn cached(&self, repo: &str, prereleases: bool) -> Option<LatestRelease>;
    /// API refused until this time (Unix seconds).
    fn rate_limited_until(&self) -> Option<i64>;
    /// Persist anything worth keeping.
    fn save(&self);
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Entry {
    etag: Option<String>,
    /// `None`: the repo has no releases.
    release: Option<Release>,
    prereleases: bool,
    fetched_at: i64,
}

#[derive(Default, Serialize, Deserialize)]
struct CacheFile {
    entries: BTreeMap<String, Entry>,
}

pub struct GitHubSource {
    http: Http,
    cache: Mutex<CacheFile>,
    cache_path: std::path::PathBuf,
    limited_until: Mutex<Option<i64>>,
}

impl GitHubSource {
    pub fn new(http: Http, cache_path: std::path::PathBuf) -> Self {
        let cache = std::fs::read(&cache_path).ok().and_then(|b| serde_json::from_slice::<CacheFile>(&b).ok()).unwrap_or_default();
        Self { http, cache: Mutex::new(cache), cache_path, limited_until: Mutex::new(None) }
    }

    fn entry(&self, repo: &str, prereleases: bool) -> Option<Entry> {
        let cache = self.cache.lock().unwrap_or_else(PoisonError::into_inner);
        cache.entries.get(repo).filter(|e| e.prereleases == prereleases).cloned()
    }

    fn store(&self, repo: &str, entry: Entry) {
        self.cache.lock().unwrap_or_else(PoisonError::into_inner).entries.insert(repo.to_string(), entry);
    }

    fn limited(&self) -> bool {
        let until = *self.limited_until.lock().unwrap_or_else(PoisonError::into_inner);
        until.is_some_and(|u| u > crate::now())
    }

    fn ask_api(&self, repo: &str, prereleases: bool) -> Result<Option<Answer>, String> {
        let url = if prereleases {
            format!("https://api.github.com/repos/{repo}/releases?per_page=30")
        } else {
            format!("https://api.github.com/repos/{repo}/releases/latest")
        };
        let saved = self.entry(repo, prereleases);
        let found =
            |release: Option<Release>| Answer { latest: release.map_or(LatestRelease::None, LatestRelease::Found), source: Source::GitHubApi };
        match self.http.get_api(&url, saved.as_ref().and_then(|e| e.etag.as_deref()))? {
            Fetched::Body { text, etag } => {
                let release = if prereleases {
                    release::parse_github_release_list(&text, repo, true)?
                } else {
                    Some(release::parse_github_release(&text, repo)?)
                };
                self.store(repo, Entry { etag, release: release.clone(), prereleases, fetched_at: crate::now() });
                Ok(Some(found(release)))
            }
            Fetched::NotModified => match saved {
                Some(e) => {
                    self.store(repo, Entry { fetched_at: crate::now(), ..e.clone() });
                    Ok(Some(found(e.release)))
                }
                None => Err("GitHub said nothing changed, but there's no saved answer".into()),
            },
            Fetched::NotFound => {
                self.store(repo, Entry { etag: None, release: None, prereleases, fetched_at: crate::now() });
                Ok(Some(found(None)))
            }
            Fetched::RateLimited { reset } => {
                *self.limited_until.lock().unwrap_or_else(PoisonError::into_inner) = Some(reset.unwrap_or_else(|| crate::now() + 15 * 60));
                Ok(None)
            }
        }
    }
}

impl ReleaseSource for GitHubSource {
    fn latest(&self, repo: &str, prereleases: bool) -> Answer {
        let api_error = if self.limited() {
            None
        } else {
            match self.ask_api(repo, prereleases) {
                Ok(Some(answer)) => return answer,
                Ok(None) => None,
                Err(e) => Some(e),
            }
        };
        let why = api_error.unwrap_or_else(|| "GitHub's request limit was reached".into());
        match self.cached(repo, prereleases) {
            Some(latest) => Answer { latest, source: Source::Cache },
            None => Answer { latest: LatestRelease::Failed(why), source: Source::Cache },
        }
    }

    fn cached(&self, repo: &str, prereleases: bool) -> Option<LatestRelease> {
        self.entry(repo, prereleases).map(|e| e.release.map_or(LatestRelease::None, LatestRelease::Found))
    }

    fn rate_limited_until(&self) -> Option<i64> {
        let until = *self.limited_until.lock().unwrap_or_else(PoisonError::into_inner);
        until.filter(|u| *u > crate::now())
    }

    fn save(&self) {
        let json = {
            let cache = self.cache.lock().unwrap_or_else(PoisonError::into_inner);
            serde_json::to_vec(&*cache)
        };
        match json {
            Ok(bytes) => {
                if let Err(e) = crate::paths::write_atomic(&self.cache_path, &bytes) {
                    log::warn!("release cache: {e}");
                }
            }
            Err(e) => log::warn!("release cache: {e}"),
        }
    }
}
