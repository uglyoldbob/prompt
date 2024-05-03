use eframe::egui;

use prompt::EguiPrompting;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Demo egui app",
        options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
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

#[derive(Default)]
struct MyEguiApp {
    test: Test,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.test.val_pb2.title = Some("Test title".to_string());
            self.test.val_pb2.filter = Some(("Test file".to_string(), vec!["*.txt".to_string()]));
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.heading("Hello World!");
                    let _ = self.test.build_gui(ui, None);
                });
        });
    }
}
