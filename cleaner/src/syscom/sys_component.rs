use anyhow::Result;
use anyhow::anyhow;
use std::ffi::CStr;
use std::path::Path;
use std::path::PathBuf;
// ============
use objc2::rc::Retained;
use objc2::{ClassType, msg_send};
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSArray;
use objc2_foundation::{NSError, NSFileManager, NSString, NSURL};
// ============
use libc::confstr;
use libc::{SIGTERM, c_int, kill};

pub const DARWIN_USER_CACHE_DIR: i32 = libc::_CS_DARWIN_USER_CACHE_DIR;
pub const DARWIN_USER_TEMP_DIR: i32 = libc::_CS_DARWIN_USER_TEMP_DIR;

pub fn sysconf_path(name: i32) -> Option<PathBuf> {
    unsafe {
        // First call: get required buffer size
        let len = confstr(name, std::ptr::null_mut(), 0);
        if len == 0 {
            return None;
        }

        let mut buf = vec![0u8; len as usize];

        // Second call: fill buffer
        let written = confstr(name, buf.as_mut_ptr() as *mut _, len);
        if written == 0 {
            return None;
        }

        let s = CStr::from_ptr(buf.as_ptr() as *const _)
            .to_string_lossy()
            .trim() // remove newline if any
            .trim_end_matches('/') // match bash sed
            .to_string();

        Some(PathBuf::from(s))
    }
}

pub fn kill_pids(pids: &str) -> Result<()> {
    for pid_str in pids.split_whitespace() {
        // parse PID
        let pid = pid_str
            .parse::<i32>()
            .map_err(|_| anyhow!("Invalid PID: {}", pid_str))?;

        // call libc::kill
        let ret = unsafe { kill(pid as c_int, SIGTERM) };

        if ret != 0 {
            // errno contains the error code
            let err = std::io::Error::last_os_error();
            return Err(anyhow!("Failed to kill PID {}: {}", pid, err));
        }
    }
    Ok(())
}

pub fn trash_files_nsfilemanager(paths: &[PathBuf]) -> Result<Vec<(PathBuf, String)>> {
    let mut failed_paths = Vec::new();

    if paths.is_empty() {
        return Ok(failed_paths);
    }

    unsafe {
        // NSFileManager *fm = [NSFileManager defaultManager]
        let fm: Retained<NSFileManager> = msg_send![NSFileManager::class(), defaultManager];

        let urls: Vec<Retained<NSURL>> = paths
            .iter()
            .filter_map(|path| {
                let s = path.to_str()?;
                let ns_string = NSString::from_str(s);
                let url: Retained<NSURL> = msg_send![NSURL::class(), fileURLWithPath: &*ns_string];
                Some(url)
            })
            .collect();

        for (i, url) in urls.iter().enumerate() {
            let mut resulting_url: *mut NSURL = std::ptr::null_mut();
            let mut error: *mut NSError = std::ptr::null_mut();

            let success: bool = msg_send![
                &*fm,
                trashItemAtURL: &**url,
                resultingItemURL: &mut resulting_url,
                error: &mut error
            ];

            if !success {
                let reason = if !error.is_null() {
                    let domain = (*error).domain().to_string();
                    let code = (*error).code();
                    if domain == "NSCocoaErrorDomain" && code == 513 {
                        "Permission not allowed by macOS privacy protection (TCC)".to_string()
                    } else {
                        format!("Failed with {} ({})", domain, code)
                    }
                } else {
                    "unknown reason".to_string()
                };

                failed_paths.push((paths[i].clone(), reason));
            }
        }
    }

    Ok(failed_paths)
}

pub fn show_in_finder(path: &Path) -> Result<()> {
    let s = path
        .to_str()
        .ok_or_else(|| anyhow!("Path is not valid UTF-8"))?;

    let ns_path = NSString::from_str(s);
    let url = NSURL::fileURLWithPath(&ns_path);
    let urls = NSArray::from_slice(&[&*url]);
    let workspace = NSWorkspace::sharedWorkspace();

    unsafe {
        let _: () = msg_send![&workspace, activateFileViewerSelectingURLs: &*urls];
    }

    Ok(())
}
