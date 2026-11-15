use std::fmt;

use bitflags::bitflags;

bitflags! {
    /// `renameat2` flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RenameFlags: u32 {
        /// Don't overwrite newpath of the rename.
        #[cfg(target_os = "linux")]
        const RENAME_NOREPLACE = libc::RENAME_NOREPLACE;
        /// Atomically exchange oldpath and newpath.
        #[cfg(target_os = "linux")]
        const RENAME_EXCHANGE = libc::RENAME_EXCHANGE;
        /// Overlay/union-specific operation.
        #[cfg(target_os = "linux")]
        const RENAME_WHITEOUT = libc::RENAME_WHITEOUT;
        // Android(bionic)では`libc`クレートの`RENAME_*`がi32型(glibcはu32)。値はLinuxカーネルと同一。
        #[cfg(target_os = "android")]
        const RENAME_NOREPLACE = libc::RENAME_NOREPLACE as u32;
        #[cfg(target_os = "android")]
        const RENAME_EXCHANGE = libc::RENAME_EXCHANGE as u32;
        #[cfg(target_os = "android")]
        const RENAME_WHITEOUT = libc::RENAME_WHITEOUT as u32;
    }
}

impl fmt::Display for RenameFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.bits(), f)
    }
}
