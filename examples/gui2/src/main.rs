pub mod egui_multiwin_dynamic {
    egui_multiwin::tracked_window!(crate::AppCommon, crate::CustomEvent, crate::MyWindows);
    egui_multiwin::multi_window!(crate::AppCommon, crate::CustomEvent, crate::MyWindows);
}

use prompt::EguiPrompting;

#[enum_dispatch(TrackedWindow)]
pub enum MyWindows {
    Popup(PopupWindow),
}

use egui_multiwin::arboard;
use egui_multiwin::egui;
use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::enum_dispatch::enum_dispatch;
use egui_multiwin_dynamic::multi_window::NewWindowRequest;
use egui_multiwin_dynamic::tracked_window::RedrawResponse;
use egui_multiwin_dynamic::tracked_window::TrackedWindow;
use std::sync::Arc;

pub struct AppCommon {
    clicks: u32,
}

#[derive(Debug)]
pub struct CustomEvent {
    window: Option<egui_multiwin::winit::window::WindowId>,
    message: u32,
}

impl CustomEvent {
    fn window_id(&self) -> Option<egui_multiwin::winit::window::WindowId> {
        self.window
    }
}

#[derive(Default, EguiPrompting)]
pub enum TestEnum {
    #[default]
    Option1,
    Option2,
    Option3 {
        asdf: u8,
        fdsa: u8,
    },
    Option4(u8, u8),
}

#[derive(Default, EguiPrompting)]
pub struct Test {
    booltest: bool,
    string: String,
    val_u8: u8,
    test_enum1: TestEnum,
    val_i8: i8,
    val_u16: u16,
    val_i16: i16,
    val_u32: u32,
    val_i32: i32,
    val_u64: u64,
    val_i64: i64,
    val_usize: usize,
    val_isize: isize,
    val_f32: f32,
    val_f64: f64,
    val_pb: std::path::PathBuf,
    val_pb3: prompt::FileCreate,
    vec_u8: Vec<u8>,
    optional: Option<String>,
    boxed_string: Box<String>,
    hm: prompt::SelectedHashMap<String>,
    pw: prompt::Password,
    pw2: prompt::Password2,
    val_pb2: prompt::FileOpen,
}

pub struct PopupWindow {
    test: Test,
}

impl PopupWindow {
    pub fn request() -> NewWindowRequest {
        NewWindowRequest {
            window_state: MyWindows::Popup(PopupWindow {
                test: Test::default(),
            }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(false)
                .with_inner_size(egui_multiwin::winit::dpi::LogicalSize {
                    width: 400.0,
                    height: 200.0,
                })
                .with_title("A window"),
            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
            id: egui_multiwin::multi_window::new_id(),
        }
    }
}

impl TrackedWindow for PopupWindow {
    fn is_root(&self) -> bool {
        true
    }

    fn redraw(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
        _window: &egui_multiwin::winit::window::Window,
        _clipboard: &mut arboard::Clipboard,
    ) -> RedrawResponse {
        let quit = false;
        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            self.test.val_pb2.title = Some("Test title".to_string());
            self.test.val_pb2.filter = Some(("Test file".to_string(), vec!["*.txt".to_string()]));
            egui_multiwin::egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| match self.test.build_gui(ui, Some("test")) {
                    Ok(_) => {
                        if ui.button("Submit").clicked() {
                            println!("Clicked submit");
                        }
                    }
                    Err(e) => {
                        ui.label(format!("Failed to validate: {}", e));
                    }
                });
        });
        RedrawResponse {
            quit,
            new_windows: Vec::new(),
        }
    }
}

impl AppCommon {
    fn process_event(&mut self, event: CustomEvent) -> Vec<NewWindowRequest> {
        let mut windows_to_create = vec![];
        println!("Received an event {:?}", event);
        if event.message == 42 {
            windows_to_create.push(PopupWindow::request());
        }
        windows_to_create
    }
}

fn main() {
    egui_multiwin_dynamic::multi_window::MultiWindow::start(|multi_window, event_loop, _proxy| {
        let root_window = PopupWindow::request();

        let mut ac = AppCommon { clicks: 0 };

        if let Err(e) = multi_window.add(root_window, &mut ac, event_loop) {
            println!("Failed to create main window {:?}", e);
        }
        ac
    })
    .unwrap();
}
