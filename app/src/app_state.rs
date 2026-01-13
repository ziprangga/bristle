use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

use crate::app_modal::{ModalAsk, ModalAskMessage};
use crate::app_status::{Status, StatusMessage};
use cleaner::Cleaner;

#[derive(Debug, Clone)]
pub enum AppMessage {
    DropFile(PathBuf),
    InputFile,
    ScanApp(Result<Cleaner, String>),

    ModalAsk(ModalAskMessage),
    ConfirmKill(Result<Cleaner, String>),

    UpdateCleaner(Cleaner),
    OpenSelectedPath(usize),

    BrowseOutput,
    OutputFile(Result<Arc<PathBuf>, String>),
    ExportFile,

    TrashApp,
    DeletedApp(Result<Vec<(PathBuf, String)>, String>),
    ClearList,
    Status(StatusMessage),

    NoOperations,
}

#[derive(Clone)]
pub struct AppState {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub status: Status,

    pub cleaner: Cleaner,
    pub selected_file: Option<usize>,
    pub show_modal_ask: ModalAsk,
    pub pending_cleaner: Option<Cleaner>,
}

impl AppState {
    pub fn new() -> Self {
        let input_file = PathBuf::new();
        let output_file = PathBuf::new();
        let status = Status {
            message: None,
            event: None,
            show_percentage: false,
        };

        let cleaner = Cleaner::default();
        let selected_file = None;

        let show_modal_ask = ModalAsk::default();
        let pending_cleaner = None;

        Self {
            input_file,
            output_file,
            status,
            cleaner,
            selected_file,
            show_modal_ask,
            pending_cleaner,
        }
    }

    pub fn reset(&mut self) {
        self.input_file.clear();
        self.output_file.clear();
        self.cleaner.reset();
        self.selected_file = None;
        self.status.reset();
    }
}
