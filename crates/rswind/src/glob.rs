use std::path::{Component, Path, PathBuf};

use either::Either;
use globset::{Glob, GlobSet, GlobSetBuilder};
use lazy_static::lazy_static;
use rayon::iter::ParallelIterator;
use rswind_common::iter::MaybeParallelIterator;
use thiserror::Error;

lazy_static! {
    pub static ref DEFAULT_GLOB: &'static [&'static str] =
        &["./**/*.{html,js,jsx,mjs,cjs,ts,tsx,mts,cts,vue,svelte,mdx}"];
}

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("Invalid path prefix: {0:?}")]
    InvalidPath(PathBuf),
    #[error("Parent directory(..) is not allowed")]
    ParentDirNotAllowed,
    #[error("Empty path")]
    EmptyPath,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Make `base` as root directory and resolve `path` to it.
fn resolve_base(path: &str, mut base: PathBuf) -> Result<PathBuf, ResolveError> {
    let components = Path::new(path).components();
    let (first, rest) = {
        let mut c = components.clone();
        (c.next().ok_or(ResolveError::EmptyPath)?, c)
    };

    match first {
        Component::CurDir => {
            base.push(rest);
            Ok(base)
        }
        Component::ParentDir => Err(ResolveError::ParentDirNotAllowed),
        Component::Normal(_) => {
            base.push(path);
            Ok(base)
        }
        Component::RootDir => {
            base.push(rest);
            Ok(base)
        }
        Component::Prefix(p) => Err(ResolveError::InvalidPath(p.as_os_str().into())),
    }
}

#[derive(Debug, Error)]
pub enum BuildGlobError {
    #[error("Glob error: {0}")]
    Glob(#[from] globset::Error),
    #[error("Resolve error: {0}")]
    Resolve(#[from] ResolveError),
    #[error("Path is not a valid UTF-8 string")]
    InvalidEncoding,
}

#[derive(Debug, Clone)]
pub struct GlobMatcher {
    glob: GlobSet,
    base: PathBuf,
}

impl GlobMatcher {
    pub fn new<T: AsRef<str>>(
        glob: impl IntoIterator<Item = T>,
        base: PathBuf,
    ) -> Result<Self, BuildGlobError> {
        let cwd = base.canonicalize().map_err(|e| BuildGlobError::Resolve(e.into()))?;

        let glob_set = glob.into_iter().try_fold(GlobSetBuilder::new(), |mut acc, g| {
            let resolved = resolve_base(g.as_ref(), cwd.clone())?;
            acc.add(Glob::new(
                resolved.as_os_str().to_str().ok_or(BuildGlobError::InvalidEncoding)?,
            )?);
            Ok::<_, BuildGlobError>(acc)
        })?;

        Ok(Self { glob: glob_set.build()?, base })
    }

    pub fn default_glob(base: PathBuf) -> Result<Self, BuildGlobError> {
        Self::new(DEFAULT_GLOB.iter().copied(), base)
    }

    pub fn is_match(&self, path: &Path) -> bool {
        self.glob.is_match(path)
    }

    pub fn base(&self) -> &Path {
        &self.base
    }
}

pub trait GlobFilter<T> {
    fn glob_filter(self, matcher: &GlobMatcher) -> impl Iterator<Item = T>;
}

impl<I, T> GlobFilter<T> for I
where
    I: Iterator<Item = T>,
    T: AsRef<Path>,
{
    fn glob_filter(self, matcher: &GlobMatcher) -> impl Iterator<Item = T> {
        self.filter(|p| matcher.is_match(p.as_ref()))
    }
}

pub trait ParallelGlobFilter<T: Send> {
    fn glob_filter(self, matcher: &GlobMatcher) -> impl ParallelIterator<Item = T>;
}

impl<I, T> ParallelGlobFilter<T> for I
where
    I: ParallelIterator<Item = T>,
    T: AsRef<Path> + Send,
{
    fn glob_filter(self, matcher: &GlobMatcher) -> impl ParallelIterator<Item = T> {
        self.filter(|p| matcher.is_match(p.as_ref()))
    }
}

pub trait MaybeParallelGlobFilter<T: Send> {
    fn glob_filter(
        self,
        matcher: &GlobMatcher,
    ) -> Either<impl Iterator<Item = T>, impl ParallelIterator<Item = T>>;
}

impl<I, T> MaybeParallelGlobFilter<T> for I
where
    I: MaybeParallelIterator<Item = T>,
    T: AsRef<Path> + Send,
{
    fn glob_filter(
        self,
        matcher: &GlobMatcher,
    ) -> Either<impl Iterator<Item = T>, impl ParallelIterator<Item = T>> {
        self.filter(|p| matcher.is_match(p.as_ref()))
    }
}
