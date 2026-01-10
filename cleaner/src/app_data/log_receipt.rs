use anyhow::{Context, Result};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

use crate::foundation::run_lsbom_command;
use crate::{AppInfo, LocationsScan};

#[derive(Debug, Default, Clone)]
pub struct LogReceipt {
    pub bom_file: Vec<PathBuf>,
}

impl LogReceipt {
    /// Find BOM files for the given app
    pub fn find_bom_files(app: &AppInfo, locations: &LocationsScan) -> Self {
        let mut bom_files = Vec::new();
        for dir in locations.receipts_dirs() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.extension().map(|ext| ext == "bom").unwrap_or(false)
                        && path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| {
                                n.contains(&app.bundle_name)
                                    || n.contains(&app.bundle_id)
                                    || n.contains(&app.name)
                                    || n.contains(&app.organization)
                            })
                            .unwrap_or(false)
                    {
                        bom_files.push(path);
                    }
                }
            }
        }

        Self {
            bom_file: bom_files,
        }
    }

    //// Save all BOM files to the given log directory in parallel
    pub fn save_bom_log(&self, log_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(log_dir)
            .with_context(|| format!("Failed to create log folder: {}", log_dir.display()))?;

        // Use par_iter() for parallel processing
        let results: Vec<Result<()>> = self
            .bom_file
            .par_iter()
            .map(|bom_file| {
                let output_file = bom_file
                    .file_name()
                    .map(|n| log_dir.join(n).with_extension("log"))
                    .context("BOM file has no filename")?;

                run_lsbom_command(bom_file, &output_file)
            })
            .collect();

        // Collect all errors, return the first one if any
        results.into_iter().collect::<Result<()>>()
    }
}
