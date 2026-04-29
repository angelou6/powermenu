using GLib;
using Gtk;
using GtkLayerShell;

class Powermenu : Gtk.Application {
  static string? config_location = null;
  static string? styles_location = null;

  const OptionEntry[] options = {
    { 
      "config",
      'c',
      OptionFlags.NONE,
      OptionArg.STRING,
      ref config_location,
      "Location of config.ini",
      "PATH",
    },
    {
      "styles",
      's',
      OptionFlags.NONE,
      OptionArg.STRING,
      ref styles_location,
      "Location of style.css", 
      "PATH",
    },
    { null }
  };

  private Gtk.Box build_box() {
    var items = parse_config (config_location);
    var box = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 0);

    foreach (var item in items) {
      var button = new Gtk.Button.with_label (item.icon);
      button.width_request = 100;
      button.height_request = 100;

      button.add_css_class (item.name);
      button.clicked.connect (() => Posix.system (item.command));

      box.append (button);
    }

    return box;
  }

  private void set_css (string location) {
    string style_path = Path.build_filename (location);
    Gtk.CssProvider css_provider = new Gtk.CssProvider ();
    css_provider.load_from_path (style_path);
    
    Gtk.StyleContext.add_provider_for_display (
      Gdk.Display.get_default (), 
      css_provider, 
      Gtk.STYLE_PROVIDER_PRIORITY_USER
    );
  }

  private Gtk.EventControllerKey create_controller (ApplicationWindow window) {
    var controller = new Gtk.EventControllerKey();

    controller.key_pressed.connect ((keyval, keycode, state) => {
      switch (keyval) {
        case Gdk.Key.Escape:
        case Gdk.Key.q:
          window.close();
          return true;

        case Gdk.Key.h:
          window.child_focus(Gtk.DirectionType.LEFT);
          return true;

        case Gdk.Key.j:
          window.child_focus(Gtk.DirectionType.DOWN);
          return true;

        case Gdk.Key.k:
          window.child_focus(Gtk.DirectionType.UP);
          return true;

        case Gdk.Key.l:
          window.child_focus(Gtk.DirectionType.RIGHT);
          return true;

        default:
          return false;
      }
    });

    return controller;
  }

  public override void activate () {
    var window = new ApplicationWindow (this);

    GtkLayerShell.init_for_window (window);
    GtkLayerShell.set_layer (window, GtkLayerShell.Layer.OVERLAY);
    GtkLayerShell.set_keyboard_mode(window, GtkLayerShell.KeyboardMode.EXCLUSIVE);

    if (FileUtils.test (styles_location, FileTest.EXISTS)) set_css (styles_location);

    window.child = build_box ();
    ((Gtk.Widget) window).add_controller (create_controller (window));
    window.present ();
  }

  public static int main (string[] args) {
    config_location = Path.build_filename(Environment.get_user_config_dir(), "powermenu", "config.ini");
    styles_location = Path.build_filename(Environment.get_user_config_dir(), "powermenu", "style.css");

    try {
      var ctx = new OptionContext ("A simple powermenu");
      ctx.set_help_enabled (true);
      ctx.add_main_entries (options, null);
      ctx.parse (ref args);
    } catch (OptionError e) {
      stdout.printf ("error: %s\n", e.message);
      stdout.printf ("Run '%s --help' to see usage.\n", args[0]);
      return 1;
    }

    if (!FileUtils.test (config_location, FileTest.EXISTS)) {
      stderr.printf ("File does not exists in location %s\n", config_location);
      return 1;
    }

    return new Powermenu ().run (args);
  }
}

