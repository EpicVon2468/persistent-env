#![cfg(windows)]
use windows_registry::Key;

use crate::Result;

// https://stackoverflow.com/questions/79701236/what-is-the-recommended-way-to-append-a-path-to-windows-path-environment-vari
// "To programmatically add or modify system environment variables, add them to the HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\Session Manager\Environment registry key"
// https://learn.microsoft.com/en-gb/windows/win32/procthread/environment-variables

macro_rules! open_env {
	($holder:ident) => {
		windows_registry::$holder
			.options()
			.write()
			.open("Environment")
	};
}

macro_rules! open_env_or_fallback {
	() => {
		open_env!(LOCAL_MACHINE).unwrap_or_else(|_| {
			open_env!(CURRENT_USER)
				.context("Couldn't get Environment for HKEY_LOCAL_MACHINE or HKEY_CURRENT_USER")
		});
	};
}

pub(crate) unsafe fn set_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	let environment: Key = open_env_or_fallback!();
	if let Err(error) = environment.set_string(key, value)? {
		// TODO: verbose error handling
		// TODO: "broadcast a WM_SETTINGCHANGE message with lParam set to the string "Environment". This allows applications, such as the shell, to pick up your updates."
		Err(Box::new(error))
	} else {
		Ok(true)
	}
}

pub(crate) unsafe fn prepend_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	Ok(false)
}

pub(crate) unsafe fn append_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	Ok(false)
}
