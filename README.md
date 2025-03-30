# Basic hot reload template with egui

* Launcher gets the `update` function from a library
* If the library has been updated, the library is replaced in memory
* Modify the app in `app/src/lib.rs`

## Execute the launcher

```
cd launcher
cargo run -r
```

## Modify the app logic

```
cd app
cargo watch -x 'build -r'
```
