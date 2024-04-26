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

#[derive(Default)]
pub enum TestEnum2 {
    #[default]
    Option1,
    Option2,
    Option3 {
        asdf: u8,
        fdsa: u8,
    },
}

impl prompt::EguiPrompting for TestEnum2 {
    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
        let combobox = if let Some(name) = name {
            let mut s = "Select a ".to_string();
            s.push_str(&name);
            egui::ComboBox::from_label(s)
        } else {
            egui::ComboBox::from_label("Select")
        };
        let val = match self {
            Self::Option1 => "Option1",
            Self::Option2 => "Option2",
            Self::Option3 { .. } => "Option3",
        };
        combobox.selected_text(val).show_ui(ui, |ui| {
            if ui.selectable_label(false, "Option1").clicked() {
                *self = Self::Option1;
            }
            if ui.selectable_label(false, "Option2").clicked() {
                *self = Self::Option2;
            }
            if ui.selectable_label(false, "Option3").clicked() {
                *self = Self::Option3 { asdf: 0, fdsa: 0 };
            }
        });
        Ok(())
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
}

#[derive(Default, EguiPrompting)]
pub struct Test {
    string: String,
    pw: prompt::Password,
    pw2: prompt::Password2,
    val_u8: u8,
    test_enum1: TestEnum,
    test_enum2: TestEnum2,
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
