#![cfg(target_os = "macos")]
use crate::Result;

pub(crate) unsafe fn set_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	let (key, value): (&str, &str) = (key.as_ref(), value.as_ref());
	Ok(false)
}

pub(crate) unsafe fn prepend_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	let (key, value): (&str, &str) = (key.as_ref(), value.as_ref());
	Ok(false)
}

pub(crate) unsafe fn append_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	let (key, value): (&str, &str) = (key.as_ref(), value.as_ref());
	Ok(false)
}
