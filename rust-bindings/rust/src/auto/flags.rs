// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;

#[cfg(any(feature = "v2015_7", feature = "dox"))]
bitflags! {
    pub struct RepoCommitState: u32 {
        const NORMAL = 0;
        const PARTIAL = 1;
    }
}

#[cfg(any(feature = "v2015_7", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for RepoCommitState {
    type GlibType = ffi::OstreeRepoCommitState;

    fn to_glib(&self) -> ffi::OstreeRepoCommitState {
        self.bits()
    }
}

#[cfg(any(feature = "v2015_7", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoCommitState> for RepoCommitState {
    fn from_glib(value: ffi::OstreeRepoCommitState) -> RepoCommitState {
        RepoCommitState::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoListRefsExtFlags: u32 {
        const NONE = 0;
        const ALIASES = 1;
        const EXCLUDE_REMOTES = 2;
        const EXCLUDE_MIRRORS = 4;
    }
}

#[doc(hidden)]
impl ToGlib for RepoListRefsExtFlags {
    type GlibType = ffi::OstreeRepoListRefsExtFlags;

    fn to_glib(&self) -> ffi::OstreeRepoListRefsExtFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoListRefsExtFlags> for RepoListRefsExtFlags {
    fn from_glib(value: ffi::OstreeRepoListRefsExtFlags) -> RepoListRefsExtFlags {
        RepoListRefsExtFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoPullFlags: u32 {
        const NONE = 0;
        const MIRROR = 1;
        const COMMIT_ONLY = 2;
        const UNTRUSTED = 4;
        const BAREUSERONLY_FILES = 8;
        const TRUSTED_HTTP = 16;
    }
}

#[doc(hidden)]
impl ToGlib for RepoPullFlags {
    type GlibType = ffi::OstreeRepoPullFlags;

    fn to_glib(&self) -> ffi::OstreeRepoPullFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoPullFlags> for RepoPullFlags {
    fn from_glib(value: ffi::OstreeRepoPullFlags) -> RepoPullFlags {
        RepoPullFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RepoResolveRevExtFlags: u32 {
        const NONE = 0;
        const LOCAL_ONLY = 1;
    }
}

#[doc(hidden)]
impl ToGlib for RepoResolveRevExtFlags {
    type GlibType = ffi::OstreeRepoResolveRevExtFlags;

    fn to_glib(&self) -> ffi::OstreeRepoResolveRevExtFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeRepoResolveRevExtFlags> for RepoResolveRevExtFlags {
    fn from_glib(value: ffi::OstreeRepoResolveRevExtFlags) -> RepoResolveRevExtFlags {
        RepoResolveRevExtFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct SePolicyRestoreconFlags: u32 {
        const NONE = 0;
        const ALLOW_NOLABEL = 1;
        const KEEP_EXISTING = 2;
    }
}

#[doc(hidden)]
impl ToGlib for SePolicyRestoreconFlags {
    type GlibType = ffi::OstreeSePolicyRestoreconFlags;

    fn to_glib(&self) -> ffi::OstreeSePolicyRestoreconFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OstreeSePolicyRestoreconFlags> for SePolicyRestoreconFlags {
    fn from_glib(value: ffi::OstreeSePolicyRestoreconFlags) -> SePolicyRestoreconFlags {
        SePolicyRestoreconFlags::from_bits_truncate(value)
    }
}

