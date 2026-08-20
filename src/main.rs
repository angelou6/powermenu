use std::path::PathBuf;
use std::process::{self, Command};
use std::sync::OnceLock;

use anyhow::Result;
use gtk::gdk::{Display, Key};
use gtk::gio::File;
use gtk::glib::home_dir;
use gtk::{Application, ApplicationWindow, CssProvider, EventControllerKey, glib};
use gtk::{Box, Button, prelude::*};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::config::{Item, parse_config};

mod config;

struct Args {
    config: PathBuf,
    css: PathBuf,
}

static CLI_ARGS: OnceLock<Args> = OnceLock::new();

fn create_controller(window: &ApplicationWindow) -> EventControllerKey {
    let controller = EventControllerKey::builder().build();

    controller.connect_key_pressed({
        let window = window.clone();
        move |_controller, keyval, _keycode, _state| match keyval {
            Key::Escape | Key::q => {
                window.close();
                glib::Propagation::Stop
            }
            Key::h => {
                window.child_focus(gtk::DirectionType::Left);
                glib::Propagation::Stop
            }
            Key::l => {
                window.child_focus(gtk::DirectionType::Right);
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
        }
    });

    controller
}

fn load_css() {
    let args = CLI_ARGS.get().unwrap();
    let provider = CssProvider::new();
    let f = File::for_path(args.css.clone());
    provider.load_from_file(&f);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}

fn build_box() -> Result<Box, anyhow::Error> {
    let button_box = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let args = CLI_ARGS.get().unwrap();
    for (key, value) in parse_config(&args.config)? {
        let item: Item = value.clone().try_into()?;
        let btn = Button::builder()
            .width_request(100)
            .height_request(100)
            .label(&item.icon)
            .build();
        btn.add_css_class(&key);

        btn.connect_clicked(move |_| {
            if let Err(err) = (|| -> Result<()> {
                Command::new("sh").args(["-c", &item.command]).spawn()?;
                process::exit(0);
            })() {
                eprintln!("Error: {err}");
            }
        });

        button_box.append(&btn);
    }

    Ok(button_box)
}

fn activate(app: &Application) {
    let button_box = match build_box() {
        Ok(b) => b,
        Err(err) => {
            eprintln!("Error: {err:?}");
            return;
        }
    };

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Powermenu")
        .child(&button_box)
        .build();
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    button_box.add_controller(create_controller(&window));

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().build();
    app.connect_startup(|_| load_css());

    app.add_main_option(
        "config",
        glib::Char::from(b'c'),
        glib::OptionFlags::NONE,
        glib::OptionArg::Filename,
        "Path to config file",
        None,
    );

    app.add_main_option(
        "css",
        glib::Char::from(b's'),
        glib::OptionFlags::NONE,
        glib::OptionArg::Filename,
        "Path to css file",
        None,
    );

    app.connect_handle_local_options(|_app, options| {
        let mut config: PathBuf = home_dir().join(".config/powermenu/config.toml");
        let mut css: PathBuf = home_dir().join(".config/powermenu/style.css");

        if let Some(config_variant) = options.lookup_value("config", None) {
            config = config_variant.get().unwrap();
        }
        if let Some(css_variant) = options.lookup_value("css", None) {
            css = css_variant.get().unwrap();
        }

        CLI_ARGS.set(Args { config, css }).ok();
        std::ops::ControlFlow::Continue(())
    });

    app.connect_activate(activate);

    app.run()
}
