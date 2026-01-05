mod app_data;
pub use app_data::*;

use anyhow::{Context, Result};
use status::StatusEmitter;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Default, Clone)]
pub struct Cleaner {
    pub app_data: AppData,
}

impl Cleaner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_app(path: &Path, status: Option<&StatusEmitter>) -> Result<Self> {
        let mut app_data = AppData::new(path)?;

        if let Some(s) = status {
            s.with_message(format!(
                "Scanning running processes for '{}'",
                app_data.app.name
            ))
            .emit();
        }

        // Find running processes
        app_data.find_pid_and_command();

        let total_process = app_data.app_process.len();

        if let Some(s) = status {
            s.with_message(format!("Found process {}", total_process))
                .emit();
        }

        // If any processes found, show confirmation dialog
        if !app_data.app_process.is_empty() {
            let user_confirmed = Self::confirm_kill_dialog(&app_data.app.name)?;

            if user_confirmed {
                // User chose Yes → kill processes
                AppProcess::kill_app_processes(&app_data.app.name, &app_data.app_process)?;

                if let Some(s) = status {
                    s.with_stage("Completed")
                        .with_message("All processes killed")
                        .with_total(total_process)
                        .emit();
                }
            }
        }

        Ok(Self { app_data })
    }

    /// Scan an app at the given path and return AppData
    pub fn scan_app_data(&mut self, status: Option<&StatusEmitter>) -> Result<&Self> {
        if let Some(s) = status {
            s.with_message(format!(
                "Scanning logs and associated files for '{}'",
                self.app_data.app.name
            ))
            .emit();
        }

        if let Some(s) = status {
            s.with_stage("started")
                .with_message("Finding BOM logs...")
                .emit();
        }

        self.app_data.find_log_bom();

        let total_bom_file = self.app_data.log.bom_file.len();

        if let Some(s) = status {
            s.with_stage("completed")
                .with_total(total_bom_file)
                .with_message("BOM logs scan completed")
                .emit();
        }

        let locations = LocationsScan::new();

        if let Some(s) = status {
            s.with_stage("started")
                .with_message("Finding associated files...")
                .emit();
        }

        self.app_data
            .find_associate_files(&locations, |cur, _path| {
                if let Some(s) = status {
                    s.with_stage("Searching").with_current(cur).emit();
                }
            });

        if let Some(s) = status {
            s.with_stage("completed")
                .with_message("Associated files scan completed")
                .emit();
        }

        Ok(self)
    }

    /// Save BOM logs of the current app to the given folder
    pub fn save_bom_logs(&self, log_dir: Option<&Path>) -> Result<()> {
        // Determine the folder
        let base_folder: PathBuf = match log_dir {
            Some(p) => p.to_path_buf(),
            None => {
                let home = std::env::var("HOME").context("Could not determine HOME directory")?;
                Path::new(&home).join("Desktop/BOM_Logs")
            }
        };

        let app_folder = base_folder.join(&self.app_data.app.name);
        // Call the LogReceipt function
        self.app_data.log.save_bom_log(&app_folder)
    }

    /// Move all associated files including the app itself to trash
    pub fn trash_all(&self) -> Result<()> {
        let mut paths: Vec<PathBuf> = self
            .app_data
            .associate_files
            .iter()
            .map(|(path, _label)| path.clone())
            .collect();

        // include the app itself
        paths.push(self.app_data.app.path.clone());
        Self::trash_files(&paths)
    }

    /// Print a summary of the app data
    /// For CLI
    pub fn print_summary(&self) {
        println!("App Name: {}", self.app_data.app.name);
        println!("Bundle ID: {}", self.app_data.app.bundle_id);
        println!("Bundle Name: {}", self.app_data.app.bundle_name);

        println!("\nRunning processes:");
        for p in &self.app_data.app_process {
            println!("PID {}: {}", p.pid, p.command);
        }

        println!("\nLog BOM files:");
        for log in &self.app_data.log.bom_file {
            println!("{}", log.display());
        }

        println!("\nAssociated files:");
        for (_i, (path, label)) in &self.app_data.all_found_entries() {
            println!("{} -> {}", label, path.display());
        }
    }

    pub fn show_in_finder(path: &Path) -> Result<()> {
        Command::new("open").arg("-R").arg(path).status()?;

        Ok(())
    }

    /// Move  paths to trash
    pub fn trash_files(paths: &[PathBuf]) -> Result<()> {
        if paths.is_empty() {
            return Ok(());
        }

        let script = paths
            .iter()
            .map(|p| format!("POSIX file \"{}\"", p.display()))
            .collect::<Vec<_>>()
            .join(", ");

        let applescript = format!(
            "tell application \"Finder\" to move {{{}}} to trash",
            script
        );

        Command::new("osascript")
            .arg("-e")
            .arg(applescript)
            .status()?;

        Ok(())
    }

    pub fn confirm_kill_dialog(app_name: &str) -> Result<bool> {
        // AppleScript dialog with Yes/No buttons
        let script = format!(
            r#"
        display dialog "The app '{}' is still running.\nDo you want to kill its running process?\nBe careful to save your work first!" buttons {{"No", "Yes"}} default button "No"
        if button returned of result is "Yes" then
            return "YES"
        else
            return "NO"
        end if
        "#,
            app_name
        );

        let output = Command::new("osascript").arg("-e").arg(script).output()?;

        let response = String::from_utf8_lossy(&output.stdout);

        Ok(response.trim() == "YES")
    }

    pub fn reset(&mut self) {
        self.app_data.reset();
    }
}
