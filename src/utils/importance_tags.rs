/// This file contains the tags for the different levels of importance.
/// These are pasted in front of the message when logging.

/// Error tag displays red.
pub(crate) const fn error_tag() -> &'static str {
    "\x1b[31m[ERROR]\x1b[0m"
}

/// Warning tag displays yellow.
pub(crate) const fn warning_tag() -> &'static str {
    "\x1b[33m[WARNING]\x1b[0m"
}

/// Info tag displays light gray.
pub(crate) const fn info_tag() -> &'static str {
    "\x1b[37m[INFO]\x1b[0m"
}

/// Debug tag displays pink.
pub(crate) const fn debug_tag() -> &'static str {
    "\x1b[35m[DEBUG]\x1b[0m"
}
