use std::path::Path;
use std::process::Command;

use anyhow::Result;
use gtk::gdk::{Display, Key};
use gtk::gio::File;
use gtk::glib::home_dir;
use gtk::{Application, ApplicationWindow, CssProvider, EventControllerKey, glib};
use gtk::{Box, Button, prelude::*};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::config::{Item, parse_config};

mod config;

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
            // Key::j => {
            //     window.child_focus(gtk::DirectionType::Down);
            //     glib::Propagation::Stop
            // }
            // Key::k => {
            //     window.child_focus(gtk::DirectionType::Up);
            //     glib::Propagation::Stop
            // }
            _ => glib::Propagation::Proceed,
        }
    });

    controller
}

fn load_css(location: &str) {
    let provider = CssProvider::new();
    let f = File::for_path(location);
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

    let conf_path = Path::new(&home_dir())
        .join(".config")
        .join("powermenu")
        .join("config.toml");
    for (key, value) in parse_config(conf_path.to_str().unwrap())? {
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
                Ok(())
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
    let style_path = Path::new(&home_dir())
        .join(".config")
        .join("powermenu")
        .join("style.css");

    let app = Application::builder().build();
    app.connect_startup(move |_| load_css(style_path.to_str().unwrap()));
    app.connect_activate(activate);

    app.run()
}
