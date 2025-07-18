use eframe::egui;

use userprompt::EguiPrompting;

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
    #[PromptComment = "Option 1 is the best"]
    #[default]
    Option1,
    #[PromptComment = "Option 2 is an option"]
    Option2,
    #[PromptComment = "This option is nice"]
    Option3 {
        #[PromptComment = "Number 1"]
        asdf: u8,
        #[PromptComment = "Number 2"]
        fdsa: u8,
    },
    #[PromptComment = "A pair of numbers"]
    Option4(u8, u8),
}

#[derive(Default, EguiPrompting)]
pub struct Test {
    #[PromptComment = "Should we do the thing?"]
    booltest: bool,
    #[PromptComment = "What should the answer be?"]
    string: String,
    #[PromptComment = "A value between 0 and 255"]
    val_u8: u8,
    #[PromptComment = "An enum"]
    test_enum1: TestEnum,
    #[PromptComment = "A signed byte"]
    val_i8: i8,
    #[PromptComment = "An unsigned 2 byte variable"]
    val_u16: u16,
    #[PromptComment = "A signed 2 byte variable"]
    val_i16: i16,
    #[PromptComment = "An unsigned 4 byte variable"]
    val_u32: u32,
    #[PromptComment = "A signed 4 byte variable"]
    val_i32: i32,
    #[PromptComment = "An unsigned 8 byte variable"]
    val_u64: u64,
    #[PromptComment = "A signed 8 byte variable"]
    val_i64: i64,
    #[PromptComment = "A variable of optimal size"]
    val_usize: usize,
    #[PromptComment = "a signed variable of optimal size"]
    val_isize: isize,
    #[PromptComment = "A 32 bit float"]
    val_f32: f32,
    #[PromptComment = "A 64 bit float"]
    val_f64: f64,
    #[PromptComment = "A path to do stuff with"]
    val_pb: std::path::PathBuf,
    #[PromptComment = "A file to create"]
    val_pb3: userprompt::FileCreate,
    #[PromptComment = "An optional file to create"]
    val_pb4: Option<userprompt::FileCreate>,
    #[PromptComment = "Some data"]
    vec_u8: Vec<u8>,
    #[PromptComment = "Optional string"]
    optional: Option<String>,
    #[PromptComment = "Boxed string"]
    boxed_string: Box<String>,
    #[PromptComment = "map of strings"]
    hm: userprompt::SelectedHashMap<String>,
    #[PromptComment = "The password to launch nuclear missiles"]
    pw: userprompt::Password,
    #[PromptComment = "An existing file to process"]
    val_pb2: userprompt::FileOpen,
    #[PromptComment = "An optional existing file to process"]
    val_pb5: Option<userprompt::FileOpen>,
    #[PromptComment = "An optional list of files to process"]
    complex1: Option<Vec<userprompt::FileOpen>>,
    #[PromptComment = "A new password to process stuff with"]
    pw2: userprompt::Password2,
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
                    let _ = self.test.build_gui(ui, None, None);
                });
        });
    }
}
