#![windows_subsystem = "windows"]

use std::path::PathBuf;

use druid::widget::{prelude::*, SizedBox};
use druid::widget::{Align, Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox};
use druid::{
    commands, AppDelegate, AppLauncher, Color, Command, DelegateCtx, Env, FileDialogOptions,
    FileSpec, Handled, Lens, LocalizedString, PlatformError, Target, Widget, WidgetExt, WindowDesc,
};

use mojiharau::{fix_mojibake, Config};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
struct OpenSaveState {
    file_data: String,
    output_filepath: String,
    input_filepath: String,
}

struct Delegate;

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("open-save-demo").with_placeholder("Opening/Saving Demo"))
        .window_size((400.0, 400.0));
    let data = OpenSaveState {
        file_data: "Type here.".to_string(),
        output_filepath: String::new(),
        input_filepath: String::new(),
    };
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<OpenSaveState> {
    let zip = FileSpec::new("Zip file", &["zip"]);
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![zip])
        .default_type(zip)
        .name_label("Target")
        .title("Please select the file to fix")
        .button_text("Ok");
    let open_dialog_options = save_dialog_options
        .clone()
        .default_name("MySavedFile.txt")
        .name_label("Source")
        .title("Please select output path or file")
        .button_text("Ok");

    let source_path = TextBox::new()
        .lens(OpenSaveState::input_filepath)
        .expand_width();
    let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });

    let destination_path = TextBox::new()
        .lens(OpenSaveState::output_filepath)
        .expand_width();
    let save = Button::new("Save").on_click(move |ctx, data: &mut OpenSaveState, _| {
        ctx.submit_command(
            druid::commands::SHOW_SAVE_PANEL.with(
                save_dialog_options
                    .clone()
                    .default_name(data.output_filepath.clone()),
            ),
        )
    });

    let source_row = Flex::row()
        .must_fill_main_axis(true)
        .with_flex_child(source_path, 1.0)
        .with_default_spacer()
        .with_child(open)
        .padding(10.0);

    let destination_row = Flex::row()
        .must_fill_main_axis(true)
        .with_flex_child(destination_path, 1.0)
        .with_default_spacer()
        .with_child(save)
        .padding(10.0);

    let paths_col = Flex::column()
        .with_child(source_row)
        .with_default_spacer()
        .with_child(destination_row)
        .padding(10.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0);

    let test_label = Label::new("TestTest!");

    let flex_container = Flex::column()
        .with_default_spacer()
        .main_axis_alignment(MainAxisAlignment::SpaceAround)
        .with_child(paths_col)
        .with_default_spacer()
        .with_child(test_label)
        .with_default_spacer()
        .padding(10.0);
    Align::centered(SizedBox::new(flex_container))
}

impl AppDelegate<OpenSaveState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut OpenSaveState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if let Err(e) = std::fs::write(file_info.path(), &data.file_data[..]) {
                println!("Error writing file: {}", e);
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    data.file_data = s.lines().take(5).collect::<Vec<_>>().join("\n");
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}
