//! Build script: embeds the ESO Weave application icon into the Windows
//! executable so the .exe file carries the brand mark in the file manager.
//!
//! This is a no-op on non-Windows hosts. The `winresource` build dependency is
//! declared only under `cfg(windows)`, so the compiled block below and the crate
//! it uses are both present only when building on Windows; the Linux build never
//! pulls or runs it.

fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        if let Err(err) = res.compile() {
            // A missing resource compiler should not fail the whole build; the
            // exe simply ships without the embedded icon and a warning is shown.
            println!("cargo:warning=failed to embed executable icon: {err}");
        }
        println!("cargo:rerun-if-changed=assets/icon.ico");
    }
}
