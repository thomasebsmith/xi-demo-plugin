//! A demo error/warning highlighting plugin for Xi Editor.
// This file is based on the plugin found here:
//  https://github.com/scholtzan/xi-todo-highlight/
extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib;
extern crate xi_rope;

use std::path::Path;

use crate::xi_core::ConfigTable;
use crate::xi_core::annotations::AnnotationType;
use xi_plugin_lib::{mainloop, ChunkCache, Plugin, View};
use crate::xi_core::plugin_rpc::DataSpan;
use xi_rope::rope::RopeDelta;
use serde_json::json;

struct DemoPlugin {
  warning_frequency: usize,
  error_frequency:   usize
}

impl Plugin for DemoPlugin {
  type Cache = ChunkCache;

  fn new_view(&mut self, view: &mut View<Self::Cache>) {
    self.create_errors_and_warnings(view);
  }

  fn did_close(&mut self, _unused_view: &View<Self::Cache>) {}
  fn did_save(
    &mut self,
    _unused_view: &mut View<Self::Cache>,
    _unused_old: Option<&Path>
  ) {}
  fn config_changed(
    &mut self,
    _unused_view: &mut View<Self::Cache>,
    _unused_changes: &ConfigTable
  ) {}

  fn update(
    &mut self,
    view: &mut View<Self::Cache>,
    _unused_delta: Option<&RopeDelta>,
    _unused_edit_type: String,
    _unused_author: String
  ) {
    self.create_errors_and_warnings(view);
  }
}

impl DemoPlugin {
  fn new() -> DemoPlugin {
    DemoPlugin {
      warning_frequency: 5,
      error_frequency:   9
    }
  }

  fn create_errors_and_warnings(&mut self, view: &mut View<ChunkCache>) {
    let first_line = view.line_of_offset(0).expect("Error getting first line");
    let last_line  = view.line_of_offset(view.get_buf_size()).expect(
      "Error getting last line"
    );

    let mut error_spans: Vec<DataSpan> = Vec::new();
    let mut warning_spans: Vec<DataSpan> = Vec::new();

    for line_num in first_line..=last_line {
      if line_num % self.error_frequency == 0 ||
          line_num % self.warning_frequency == 0 {
        let line_offset = view.offset_of_line(line_num).unwrap();
        let line_length = view.get_line(line_num).unwrap().len();

        if line_num % self.error_frequency == 0 { 
          error_spans.push(DataSpan {
            start: line_offset,
            end: line_offset + line_length,
            data: json!("There was a problem")
          });
        } else {
          warning_spans.push(DataSpan {
            start: line_offset,
            end: line_offset + line_length,
            data: json!("There was a problem")
          });
        }
      }
    }

    let annotation_error_type = AnnotationType::Other("error".to_string());
    let annotation_warning_type = AnnotationType::Other("warning".to_string());

    if error_spans.len() > 0 {
      view.update_annotations(
        0,
        view.get_buf_size(),
        &error_spans,
        &annotation_error_type
      );
    }
    if warning_spans.len() > 0 {
      view.update_annotations(
        0,
        view.get_buf_size(),
        &warning_spans,
        &annotation_warning_type
      );
    }
  }
}

fn main() {
  let mut plugin = DemoPlugin::new();
  mainloop(&mut plugin).unwrap();
}
