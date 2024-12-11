use std::{path::PathBuf, str::FromStr};

use druid::{
    widget::{Button, Checkbox, Flex, Label, TextBox},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};

#[derive(Default, Clone, Data, Lens)]
struct AppState {
    pack_id: String,
    release_id: String,
    client: bool,
    output: String,
}

fn build_ui() -> impl Widget<AppState> {
    let pack_id_input = TextBox::new()
        .with_placeholder("Enter the pack ID from FTB")
        .lens(AppState::pack_id)
        .fix_width(200.0)
        .padding(10.0);

    let release_id_input = TextBox::new()
        .with_placeholder("Enter the release ID from FTB")
        .lens(AppState::release_id)
        .fix_width(200.0)
        .padding(10.0);

    // Todo: Make it a lever
    let client_checkbox = Checkbox::new("Is client")
        .lens(AppState::client)
        .padding(10.0);

    let output_dir = TextBox::new()
        .with_placeholder("Enter a valid output path")
        .lens(AppState::output)
        .fix_width(200.0)
        .padding(10.0);

    let submit_button = Button::new("Download")
        .on_click(|_ctx, data: &mut AppState, _env| {
            println!("GUI received the values: \npack_id: {}\nrelease_id: {}\nclient?: {}\noutput_directory: {:?}", data.pack_id, data.release_id, data.client, PathBuf::from_str(data.output.as_str()).expect("No valid output path"));
        })
        .fix_width(100.0)
        .padding(10.0);

    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("Pack ID").padding(10.0))
                .with_child(pack_id_input),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("Release ID").padding(10.0))
                .with_child(release_id_input),
        )
        .with_child(client_checkbox)
        .with_child(
            Flex::row()
                .with_child(Label::new("Output directory").padding(10.0))
                .with_child(output_dir),
        )
        .with_child(submit_button)
        .align_vertical(druid::UnitPoint::CENTER)
        .align_horizontal(druid::UnitPoint::CENTER)
}

// Returns pack_id, release_id, client?, output_directory
pub fn open_gui() -> (u32, u32, bool, PathBuf) {
    let window = WindowDesc::new(build_ui())
        .title("FTB Downloader")
        .window_size((400.0, 300.0));

    let initial_state = AppState {
        pack_id: "".to_string(),
        release_id: "".to_string(),
        client: false,
        output: ".".to_string(),
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Couldn't launch window");

    todo!()
}
