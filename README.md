# WINDVIR
The simplest 2D simulation of wind currents
For compile and run project you must use command
```
cargo run --release
```
If you want to run project using `Wayland` compositor, then you need compile with command
```
cargo run --release --features wayland
```
If you want to use `FFI` protocol, then you need compile with command
```
cargo run --release -- --c
```
## Controls
`W | ArrowUp` - move up

`A | ArrowLeft` - move left

`S | ArrowDown` - move down

`D | ArrowRight` - move right

`Z` - zoom map

`X` - redo zoom

`Q | E` - rotate map

`V` - change vision
## Settings
You can change default settings in file `default_settings.json`

In the graphics column you can enable certain settings. If `multisampling` or  `depth buffering` is disabled, the corresponding parameters will not work
