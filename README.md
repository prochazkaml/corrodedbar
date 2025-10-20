# corrodedbar

A simple statusbar for your favourite window manager which either:

- displays the X11 root window name somewhere on screen (such as dwm), or
- uses Waybar (see below).

Oh, and it's written in Rust. 🦀

![image](https://github.com/prochazkaml/corrodedbar/assets/41787099/0cb8c87a-3c4e-4781-99d3-a393a99eb284)

([cmatrix](https://github.com/abishekvashok/cmatrix) not included.)

## Usage

To install, run:

```bash
cargo install --git https://github.com/prochazkaml/corrodedbar
```

Then, just add `corrodedbar` into your window manager's autostart script. Upon first launch, it will generate an example config file which you can edit to further suit your needs.

If you want to check out what corrodedbar has to offer, see the [example config file's documentation](https://github.com/prochazkaml/corrodedbar/blob/master/src/example.toml).

## New for 2025: Wayland support!

Well... sort of.

corrodedbar can now be started so that it outputs the current statusbar contents to standard output, where each line of output corresponds to a statusbar update.

This _just_ so happens to be the exact format that Waybar expects for custom scripts.

To add `corrodedbar` to your Waybar, just add this to your `~/.config/waybar/config`:

```
{
    ...
    "modules-right": ["custom/corrodedbar"],
    "custom/corrodedbar": {
        "exec": "corrodedbar --backend stdout"
    }
    ...
}
```

And there you have it. `corrodedbar` on Wayland.

