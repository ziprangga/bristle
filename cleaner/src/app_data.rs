mod app_info;
mod app_process;
mod locations_scan;
mod log_receipt;

pub use app_info::AppInfo;
pub use app_process::AppProcess;
pub use locations_scan::LocationsScan;
pub use log_receipt::LogReceipt;

use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

#[derive(Debug, Default, Clone)]
pub struct AppData {
    pub app: AppInfo,
    pub app_process: Vec<AppProcess>,
    pub log: LogReceipt,
    pub associate_files: Vec<(PathBuf, String)>,
}

impl AppData {
    pub fn new(app_path: &Path) -> Result<Self> {
        // Create AppInfo from path
        let app_info = AppInfo::from_path(app_path)?;

        Ok(Self {
            app: app_info,
            app_process: Vec::new(),
            log: LogReceipt {
                bom_file: Vec::new(),
            },
            associate_files: Vec::new(),
        })
    }

    pub fn find_pid_and_command(&mut self) {
        self.app_process = AppProcess::find_app_processes(&self.app);
    }

    pub fn find_log_bom(&mut self) {
        self.log = LogReceipt::find_bom_files(&self.app);
    }

    // Scan all file associate from list of location
    // for huge directory and try using walkdir + rayon
    // use in_progress as emitter status to caller
    pub fn find_associate_files<F>(&mut self, locations: &LocationsScan, in_progress: F)
    where
        F: Fn(usize, &Path) + Send + Sync,
    {
        let counter = Arc::new(AtomicUsize::new(0));
        let progress = Arc::new(in_progress);

        // Parallel
        let results: Vec<(PathBuf, String)> = locations
            .paths
            .par_iter()
            .filter(|base| base.exists())
            .map(|base| {
                WalkDir::new(base)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|entry| entry.file_type().is_file() || entry.file_type().is_dir())
                    .flat_map(|entry| {
                        let path_buf = entry.path().to_path_buf();
                        let mut matches = Vec::new();

                        if let Some(name) = path_buf.file_name().and_then(|n| n.to_str()) {
                            let name_lc = name.to_ascii_lowercase();

                            if name_lc.contains(&self.app.name.to_ascii_lowercase())
                                || name_lc.contains(&self.app.bundle_id.to_ascii_lowercase())
                                || name_lc.contains(&self.app.bundle_name.to_ascii_lowercase())
                                || name_lc.contains(&self.app.organization.to_ascii_lowercase())
                            {
                                matches.push((path_buf.clone(), name.to_string()));
                            }
                            // Batched atomic progress every 256 files
                            let n = counter.fetch_add(1, Ordering::Relaxed) + 1;
                            if n.is_multiple_of(256) {
                                progress(n, &path_buf);
                            }
                        }

                        matches.into_iter()
                    })
                    .collect::<Vec<_>>()
            })
            .reduce(Vec::new, |mut acc, v| {
                acc.extend(v);
                acc
            }); // Collect directly without per-base Vec

        // Deduplicate once at the end
        let mut seen = HashSet::new();
        self.associate_files = results
            .into_iter()
            .filter(|(p, _)| seen.insert(p.clone()))
            .collect();
    }

    // ===============GUI FOCUS==================
    pub fn all_found_entries(&self) -> Vec<(usize, (PathBuf, String))> {
        let mut result: Vec<(usize, (PathBuf, String))> = self
            .associate_files
            .iter()
            .enumerate()
            .map(|(i, (path, label))| (i, (path.clone(), label.clone())))
            .collect();

        result.push((result.len(), (self.app.path.clone(), self.app.name.clone())));

        result
    }

    pub fn reset(&mut self) {
        self.app = AppInfo::default();
        self.app_process.clear();
        self.log = LogReceipt::default();
        self.associate_files.clear();
    }
}
