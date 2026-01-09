use anyhow::{Result, anyhow};
use std::path::PathBuf;
// ============
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{ClassType, msg_send};

use objc2_foundation::{NSString, NSURL};

pub fn trash_files_nsfilemanager(paths: &[PathBuf]) -> Result<()> {
    unsafe {
        // NSFileManager *fm = [NSFileManager defaultManager]
        let fm: Retained<AnyObject> =
            msg_send![objc2_foundation::NSFileManager::class(), defaultManager];

        for path in paths {
            let s = path
                .to_str()
                .ok_or_else(|| anyhow!("Non-UTF8 path: {}", path.display()))?;

            // NSString *str = [NSString stringWithUTF8String:]
            let ns_string = NSString::from_str(s);

            // NSURL *url = [NSURL fileURLWithPath:]
            let url: Retained<NSURL> = msg_send![NSURL::class(), fileURLWithPath: &*ns_string];

            let mut resulting_url: *mut AnyObject = std::ptr::null_mut();

            let success: bool = msg_send![
                &*fm,
                trashItemAtURL: &*url,
                resultingItemURL: &mut resulting_url,
                error: std::ptr::null_mut::<*mut AnyObject>()
            ];

            if !success {
                return Err(anyhow!("Failed to trash: {}", path.display()));
            }
        }
    }

    Ok(())
}
