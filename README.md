# corrodedbar

A simple statusbar for X11 window managers which display the root window name somewhere on screen (such as dwm).

Oh, and it's written in Rust. 🦀

![image](https://github.com/prochazkaml/corrodedbar/assets/41787099/0cb8c87a-3c4e-4781-99d3-a393a99eb284)

([cmatrix](https://github.com/abishekvashok/cmatrix) not included.)

## Usage

To install, run:

```bash
cargo build --release
cp target/release/corrodedbar ~/.local/bin # or somewhere else in your $PATH
```

Then, just add `corrodedbar` into your window manager's autostart script. Upon first launch, it will generate an example config file which you can edit to further suit your needs.

If you want to check out what corrodedbar has to offer, see the [example config file's documentation](https://github.com/prochazkaml/corrodedbar/blob/master/src/example.toml).

