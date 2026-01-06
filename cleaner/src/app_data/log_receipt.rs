use crate::AppInfo;

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use rayon::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct LogReceipt {
    pub bom_file: Vec<PathBuf>,
}

impl LogReceipt {
    /// Find BOM files for the given app
    pub fn find_bom_files(app: &AppInfo) -> Self {
        let receipts_dir = Path::new("/private/var/db/receipts");
        let bom_files = if receipts_dir.exists() {
            fs::read_dir(receipts_dir)
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.extension().map(|ext| ext == "bom").unwrap_or(false)
                        && p.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| {
                                n.contains(&app.bundle_name)
                                    || n.contains(&app.bundle_id)
                                    || n.contains(&app.name)
                                    || n.contains(&app.organization)
                            })
                            .unwrap_or(false)
                })
                .collect()
        } else {
            Vec::new()
        };

        Self {
            bom_file: bom_files,
        }
    }

    // /// Save a single BOM file to the given output folder
    // pub fn save_bom_log(&self, log_dir: &Path) -> Result<()> {
    //     fs::create_dir_all(log_dir)
    //         .with_context(|| format!("Failed to create log folder: {}", log_dir.display()))?;

    //     for bom_file in &self.bom_file {
    //         let output_file = log_dir.join(format!(
    //             "{}.log",
    //             bom_file.file_name().unwrap().to_string_lossy()
    //         ));

    //         let output = Command::new("lsbom")
    //             .args(["-f", "-l", "-s", "-p", "f", bom_file.to_str().unwrap()])
    //             .output()
    //             .with_context(|| format!("Failed to run lsbom on {}", bom_file.display()))?;

    //         if output.status.success() {
    //             let mut f = File::create(&output_file)
    //                 .with_context(|| format!("Failed to create file: {}", output_file.display()))?;
    //             f.write_all(&output.stdout).with_context(|| {
    //                 format!("Failed to write BOM log: {}", output_file.display())
    //             })?;
    //             println!("Saved BOM log: {}", output_file.display());
    //         } else {
    //             eprintln!(
    //                 "lsbom failed for {}: {}",
    //                 bom_file.display(),
    //                 String::from_utf8_lossy(&output.stderr)
    //             );
    //         }
    //     }

    //     Ok(())
    // }

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

                let bom_file_str = bom_file.to_string_lossy();

                let output = Command::new("lsbom")
                    .args(["-f", "-l", "-s", "-p", "f", &bom_file_str])
                    .output()
                    .with_context(|| format!("Failed to run lsbom on {}", bom_file.display()))?;

                if output.status.success() {
                    let mut f = File::create(&output_file).with_context(|| {
                        format!("Failed to create file: {}", output_file.display())
                    })?;
                    f.write_all(&output.stdout).with_context(|| {
                        format!("Failed to write BOM log: {}", output_file.display())
                    })?;
                    println!("Saved BOM log: {}", output_file.display());
                    Ok(())
                } else {
                    anyhow::bail!(
                        "lsbom failed for {}: {}",
                        bom_file.display(),
                        String::from_utf8_lossy(&output.stderr)
                    )
                }
            })
            .collect();

        // Collect all errors, return the first one if any
        results.into_iter().collect::<Result<()>>()
    }
}
