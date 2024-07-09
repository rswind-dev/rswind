use std::{collections::BTreeMap, sync::Arc};

use derive_more::{Deref, DerefMut};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

use crate::design::CacheKey;

#[enum_dispatch]
pub trait Cache {
    fn mark_valid(&mut self, item: SmolStr);

    fn mark_valid_many(&mut self, items: impl IntoIterator<Item = SmolStr>);

    fn mark_invalid(&mut self, item: SmolStr);

    fn mark_invalid_many(&mut self, items: impl IntoIterator<Item = SmolStr>);

    fn store_style(&mut self, key: CacheKey, value: String);

    fn style_map(&self) -> &BTreeMap<CacheKey, String>;

    fn store_extra_css(&mut self, key: CacheKey, value: String);

    fn has_seen(&self, item: &str) -> bool;

    fn extra_css(&self) -> &BTreeMap<CacheKey, String>;

    fn css(&self) -> Arc<String>;

    fn store_css(&mut self, css: String);
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct GeneratorCache {
    pub need_cache: bool,
    #[deref]
    #[deref_mut]
    pub inner: CacheInner,
    pub state: CacheState,
}

impl GeneratorCache {
    pub fn new(state: CacheState) -> Self {
        let need_cache = state != CacheState::OneShot;
        Self {
            need_cache,
            state,
            inner: match need_cache {
                true => CacheInner::Cache(CacheImpl::default()),
                false => CacheInner::Noop(NoopCache::default()),
            },
        }
    }
}

#[derive(Debug)]
#[enum_dispatch(Cache)]
pub enum CacheInner {
    Noop(NoopCache),
    Cache(CacheImpl),
}

impl Default for CacheInner {
    fn default() -> Self {
        Self::Noop(NoopCache::default())
    }
}

#[derive(Debug, Default)]
pub struct CacheImpl {
    pub css: BTreeMap<CacheKey, String>,
    pub groups: BTreeMap<CacheKey, String>,
    pub valid: HashMap<SmolStr, bool>,
    pub generated_css: Arc<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CacheState {
    #[default]
    OneShot,
    FirstRun,
    Cached,
}

impl CacheState {
    pub fn is_cached(&self) -> bool {
        matches!(self, Self::Cached)
    }
    pub fn mark_cached(&mut self) {
        if *self == Self::FirstRun {
            *self = Self::Cached;
        }
    }
}

impl Cache for CacheImpl {
    fn style_map(&self) -> &BTreeMap<CacheKey, String> {
        &self.css
    }

    fn store_style(&mut self, key: CacheKey, value: String) {
        self.css.insert(key, value);
    }

    fn extra_css(&self) -> &BTreeMap<CacheKey, String> {
        &self.groups
    }

    fn store_extra_css(&mut self, key: CacheKey, value: String) {
        self.groups.insert(key, value);
    }

    fn has_seen(&self, item: &str) -> bool {
        self.valid.get(item).copied().unwrap_or(false)
    }

    fn mark_valid(&mut self, item: SmolStr) {
        self.valid.insert(item, true);
    }

    fn mark_valid_many(&mut self, items: impl IntoIterator<Item = SmolStr>) {
        for item in items {
            self.valid.insert(item, true);
        }
    }

    fn mark_invalid(&mut self, item: SmolStr) {
        self.valid.insert(item, false);
    }

    fn mark_invalid_many(&mut self, items: impl IntoIterator<Item = SmolStr>) {
        for item in items {
            self.valid.insert(item, false);
        }
    }

    fn css(&self) -> Arc<String> {
        self.generated_css.clone()
    }

    fn store_css(&mut self, css: String) {
        self.generated_css = Arc::new(css);
    }
}

#[derive(Debug, Default)]
pub struct NoopCache {
    pub groups: BTreeMap<CacheKey, String>,
}

lazy_static! {
    pub static ref EMPTY_MAP: BTreeMap<CacheKey, String> = BTreeMap::default();
}

impl Cache for NoopCache {
    fn style_map(&self) -> &BTreeMap<CacheKey, String> {
        &EMPTY_MAP
    }

    fn extra_css(&self) -> &BTreeMap<CacheKey, String> {
        &self.groups
    }

    fn store_extra_css(&mut self, key: CacheKey, value: String) {
        self.groups.insert(key, value);
    }

    fn has_seen(&self, _: &str) -> bool {
        false
    }

    fn mark_valid(&mut self, _: SmolStr) {}

    fn mark_valid_many(&mut self, _: impl IntoIterator<Item = SmolStr>) {}

    fn mark_invalid(&mut self, _: SmolStr) {}

    fn mark_invalid_many(&mut self, _: impl IntoIterator<Item = SmolStr>) {}

    fn store_style(&mut self, _: CacheKey, _: String) {}

    fn css(&self) -> Arc<String> {
        Arc::new(String::new())
    }

    fn store_css(&mut self, _: String) {}
}
