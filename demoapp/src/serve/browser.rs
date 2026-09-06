//! Opening the dev server in a browser.
//!
//! `[server] open = true`, or `anthology serve --open`, points the platform's
//! default browser at the server once it is listening. Every platform spells
//! this differently, and none of them is `xdg-open` everywhere.

use rmx::prelude::*;

use crate::{Error, Result};

/// The command that opens a URL in the default browser on this platform.
///
/// Returns the program and its leading arguments; the URL is appended.
pub fn opener() -> Option<(&'static str, &'static [&'static str])> {
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            Some(("open", &[]))
        } else if #[cfg(target_os = "windows")] {
            // `start` is a shell builtin rather than a program, and its first
            // argument is taken as the window title, so it needs an empty one.
            Some(("cmd", &["/C", "start", ""]))
        } else if #[cfg(unix)] {
            Some(("xdg-open", &[]))
        } else {
            None
        }
    }
}

/// The URL the dev server is reachable at.
pub fn server_url(port: u16) -> String {
    format!("http://localhost:{port}/")
}

/// Open `url` in the default browser.
///
/// A platform with no known opener, or a browser that fails to launch, is
/// reported rather than ignored: the user asked for this explicitly.
pub fn open(url: &str) -> Result<()> {
    let Some((program, leading)) = opener() else {
        return Err(Error::server(format!(
            "no way to open a browser on {}; visit {url}",
            std::env::consts::OS,
        )));
    };

    let shell = rmx::xshell::Shell::new().map_err(|e| Error::server(e.to_string()))?;
    rmx::xshell::cmd!(shell, "{program} {leading...} {url}")
        .quiet()
        .run()
        .map_err(|e| Error::server(format!("failed to open a browser: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_supported_platform_has_an_opener() {
        // The `else` arm exists for platforms Anthology does not know, but the
        // ones it is tested on are all covered.
        assert!(opener().is_some(), "no opener for {}", std::env::consts::OS);
    }

    #[test]
    fn the_url_points_at_the_serving_port() {
        assert_eq!(server_url(3000), "http://localhost:3000/");
        assert_eq!(server_url(8080), "http://localhost:8080/");
    }

    // `open` itself is not tested: it launches a browser, which is not
    // something a test run should do. What can go wrong without launching
    // anything -- picking no opener at all, or the wrong URL -- is above.

    #[test]
    fn the_opener_names_a_program_and_no_empty_arguments_before_the_title() {
        let (program, leading) = opener().unwrap();

        assert!(!program.is_empty());
        // Windows' `start` takes an empty title argument, and it must be last,
        // immediately before the URL.
        assert!(
            leading.iter().rev().skip(1).all(|arg| !arg.is_empty()),
            "{leading:?}",
        );
    }
}
