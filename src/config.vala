public class PowerItem {
  public string name;
  public string icon;
  public string command;

  public PowerItem(string name, string icon, string command) {
    this.name = name;
    this.icon = icon;
    this.command = command;
  }
}

GenericArray<PowerItem> parse_config(string location) {
  var file = new KeyFile ();
  var items = new GenericArray<PowerItem> ();

  try {
    file.load_from_file (location, KeyFileFlags.NONE);
    string[] groups = file.get_groups ();

    foreach (string group in groups) {
      string icon = file.get_string (group, "icon");
      string command = file.get_string (group, "command");

      items.add (new PowerItem (group, icon, command));
    }
  } catch (KeyFileError e) {
    stderr.printf ("Configuration error: %s\n", e.message);
  } catch (FileError e) {
    stderr.printf ("File error: %s\n", e.message);
  }

  return items;
}

