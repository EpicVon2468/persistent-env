#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::unnecessary_semicolon, clippy::tabs_in_doc_comments)]
#![forbid(
	clippy::undocumented_unsafe_blocks,
	clippy::multiple_unsafe_ops_per_block,
	clippy::missing_safety_doc,
	unsafe_op_in_unsafe_fn
)]
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(windows)]
mod windows;

pub type Result<T = bool> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! delegate_os {
	($function:ident, $($arg:expr),*) => {
		cfg_select! {
			target_os = "linux" => linux::$function($($arg, )*),
			target_os = "macos" => macos::$function($($arg, )*),
			windows => windows::$function($($arg,)*),
			_ => unimplemented!(),
		}
	};
}

// TODO: distinction between per-user (no root required (?)) and system-wide env modification?
// TODO: lazy_set_env_var / predicate_set_env_var?
// TODO: compare_and_set?
// TODO: remove_env_var?
/// # Safety
///
/// On Linux & macOS, the `value` passed here may be expanded by `sh`.
///
/// This means that an `value` such as `"$(rm -rf / --no-preserve-root)"` could feasibly cause harm when the environment is next evaluated (hyperbolic example).
///
/// The caller of this function assumses responsibility for inputs, and should either pass only trusted/constant input, or perform input validation beforehand to make sure no malicious inputs are passed.
pub unsafe fn set_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(key: K, value: V) -> Result {
	// SAFETY:
	// Problem(s):
	// - See function documentation.
	// Excuse(s):
	// - The caller assumes responsibility for values passed, and must validate input before use.
	unsafe { delegate_os!(set_env_var_unchecked, key, value) }
}

/// # Safety
///
/// TODO
pub unsafe fn prepend_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(key: K, value: V) -> Result {
	// SAFETY:
	// Problem(s):
	// - See function documentation.
	// Excuse(s):
	// - TODO
	unsafe { delegate_os!(prepend_env_var_unchecked, key, value) }
}

/// # Safety
///
/// TODO
pub unsafe fn append_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(key: K, value: V) -> Result {
	// SAFETY:
	// Problem(s):
	// - See function documentation.
	// Excuse(s):
	// - TODO
	unsafe { delegate_os!(append_env_var_unchecked, key, value) }
}
