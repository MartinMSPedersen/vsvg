#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::let_underscore_untyped)]

mod document_widget;
mod engine;
mod frame_history;
mod painters;
mod viewer;

pub use crate::document_widget::DocumentWidget;
pub use crate::engine::DocumentData;
use crate::viewer::Viewer;
use std::sync::Arc;
use vsvg::Document;

/// Empty viewer app for [`show()`]
struct EmptyViewerApp;

impl ViewerApp for EmptyViewerApp {}

/// Show a document in a window.
pub fn show(document: &Document) -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    let document_data = Arc::new(DocumentData::new(document));

    eframe::run_native(
        "vsvg",
        native_options,
        Box::new(move |cc| {
            let style = egui::Style {
                visuals: egui::Visuals::light(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);

            Box::new(
                Viewer::new(cc, document_data, Box::new(EmptyViewerApp {}))
                    .expect("viewer requires wgpu backend"),
            )
        }),
    )?;

    Ok(())
}

pub trait ViewerApp {
    fn setup(
        &mut self,
        _cc: &eframe::CreationContext,
        _document_widget: &mut DocumentWidget,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Main update hook
    ///
    /// Create side panels for UI and/or update the document widget.
    fn update(
        &mut self,
        _ctx: &egui::Context,
        _document_widget: &mut DocumentWidget,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

pub fn show_with_viewer_app(viewer_app: Box<dyn ViewerApp>) -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    let document_data = Arc::new(DocumentData::default());

    eframe::run_native(
        "vsvg",
        native_options,
        Box::new(move |cc| {
            let style = egui::Style {
                visuals: egui::Visuals::light(),
                ..egui::Style::default()
            };
            cc.egui_ctx.set_style(style);

            Box::new(
                Viewer::new(cc, document_data, viewer_app).expect("viewer requires wgpu backend"),
            )
        }),
    )?;

    Ok(())
}
