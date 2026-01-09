use anyhow::{Result, anyhow};
use std::path::PathBuf;
// ============
use objc2::rc::Retained;
use objc2::{ClassType, msg_send};
use objc2_foundation::{NSError, NSFileManager, NSString, NSURL};

pub fn trash_files_nsfilemanager(paths: &[PathBuf]) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    unsafe {
        // NSFileManager *fm = [NSFileManager defaultManager]
        let fm: Retained<NSFileManager> = msg_send![NSFileManager::class(), defaultManager];

        let mut failed_paths = Vec::new();

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

        if !failed_paths.is_empty() {
            let report = failed_paths
                .iter()
                .map(|(p, r)| format!("{}: {}", p.display(), r))
                .collect::<Vec<_>>()
                .join("\n");

            return Err(anyhow!("Some paths failed to trash:\n{}", report));
        }

        // for path in paths {
        //     let s = path
        //         .to_str()
        //         .ok_or_else(|| anyhow!("Non-UTF8 path: {}", path.display()))?;

        //     // NSString *str = [NSString stringWithUTF8String:]
        //     let ns_string = NSString::from_str(s);

        //     // NSURL *url = [NSURL fileURLWithPath:]
        //     let url: Retained<NSURL> = msg_send![NSURL::class(), fileURLWithPath: &*ns_string];

        //     let mut resulting_url: *mut NSURL = std::ptr::null_mut();
        //     let mut error: *mut NSError = std::ptr::null_mut();

        //     let success: bool = msg_send![
        //         &*fm,
        //         trashItemAtURL: &*url,
        //         resultingItemURL: &mut resulting_url,
        //         error: &mut error
        //     ];

        //     if !success {
        //         let reason = if !error.is_null() {
        //             // Just read NSError without taking ownership
        //             let domain = (*error).domain().to_string();
        //             let code = (*error).code();

        //             if domain == "NSCocoaErrorDomain" && code == 513 {
        //                 "Permission not allowed by macOS privacy protection (TCC)".to_string()
        //             } else {
        //                 format!("Failed with {} ({})", domain, code)
        //             }
        //         } else {
        //             "unknown reason".to_string()
        //         };

        //         return Err(anyhow!("Failed to trash {}: {}", path.display(), reason));
        //     }
        // }
    }

    Ok(())
}

// use objc2::rc::Retained;
// use objc2::runtime::AnyObject;
// use objc2::{ClassType, msg_send};

// use objc2_foundation::{NSString, NSURL};

// pub fn trash_files_nsfilemanager(paths: &[PathBuf]) -> Result<()> {
//     unsafe {
//         // NSFileManager *fm = [NSFileManager defaultManager]
//         let fm: Retained<AnyObject> =
//             msg_send![objc2_foundation::NSFileManager::class(), defaultManager];

//         for path in paths {
//             let s = path
//                 .to_str()
//                 .ok_or_else(|| anyhow!("Non-UTF8 path: {}", path.display()))?;

//             // NSString *str = [NSString stringWithUTF8String:]
//             let ns_string = NSString::from_str(s);

//             // NSURL *url = [NSURL fileURLWithPath:]
//             let url: Retained<NSURL> = msg_send![NSURL::class(), fileURLWithPath: &*ns_string];

//             let mut resulting_url: *mut AnyObject = std::ptr::null_mut();

//             let success: bool = msg_send![
//                 &*fm,
//                 trashItemAtURL: &*url,
//                 resultingItemURL: &mut resulting_url,
//                 error: std::ptr::null_mut::<*mut AnyObject>()
//             ];

//             if !success {
//                 return Err(anyhow!("Failed to trash: {}", path.display()));
//             }
//         }
//     }

//     Ok(())
// }
