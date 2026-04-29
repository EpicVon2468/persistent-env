#![cfg(target_os = "linux")]
use std::fs::{File, exists};
use std::io::Write as _;

use crate::Result;

// Despite being .sh, you don't seem to actually need to make it executable
const ENV_SCRIPT: &str = "/etc/profile.d/io-github-epicvon2468-persistent-env.sh";

macro_rules! open_env {
	() => {
		File::options().append(true).create(true).open(ENV_SCRIPT)?
	};
}

fn initialise_script() -> Result<File> {
	let existed: bool = exists(ENV_SCRIPT).unwrap_or(false);
	let mut file: File = open_env!();
	if !existed {
		writeln!(file, "#!/usr/bin/env sh")?;
		writeln!(file, "{PREPEND_FUNCTION}")?;
		writeln!(file, "{APPEND_FUNCTION}")?;
	};
	Ok(file)
}

/// ```
/// namepend_env() {
///		case ":$1:" in
///			*:"$2":*)
///				;;
///			*)
///				export $1="pend"
///				;;
///		esac
///	}
/// ```
macro_rules! pend_function {
	($name:literal, $pend:literal) => {
		concat!(
			$name,
			"pend_env() {\n",
			"\tcase \":$1:\" in\n",
			"\t\t*:\"$2\":*)\n",
			"\t\t\t;;\n",
			"\t\t*)\n",
			"\t\t\texport $1=\"",
			$pend,
			"\"\n",
			"\t\t\t;;\n",
			"\tesac\n",
			"}\n",
		)
	};
}

const PREPEND_FUNCTION: &str = pend_function!("pre", "$2:$1");
const APPEND_FUNCTION: &str = pend_function!("ap", "$1:$2");

// TODO: 'safe' version of this using PAM's `/etc/environment.d/*.conf` ? (might not be on all systems though)
pub(crate) unsafe fn set_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	let (key, value): (&str, &str) = (key.as_ref(), value.as_ref());
	let mut file: File = initialise_script()?;
	writeln!(file, "export {key}=\"{value}\"")?;
	Ok(false)
}

pub(crate) unsafe fn prepend_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	let (key, value): (&str, &str) = (key.as_ref(), value.as_ref());
	let mut file: File = initialise_script()?;
	writeln!(file, "prepend_env \"{key}\" \"{value}\"")?;
	Ok(false)
}

pub(crate) unsafe fn append_env_var_unchecked<K: AsRef<str>, V: AsRef<str>>(
	key: K,
	value: V,
) -> Result {
	let (key, value): (&str, &str) = (key.as_ref(), value.as_ref());
	let mut file: File = initialise_script()?;
	writeln!(file, "append_env \"{key}\" \"{value}\"")?;
	Ok(false)
}
