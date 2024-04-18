# WINDVIR
The simplest 2D simulation of wind currents
For compile and run project you must use command
```
RUST_LOG=windvir cargo run --release
```
If you want to use `FFI` protocol, then you need compile with command
```
RUST_LOG=windvir cargo run --release -- -c
```
## Controls
`W | ArrowUp` - move up

`A | ArrowLeft` - move left

`S | ArrowDown` - move down

`D | ArrowRight` - move right

`Z` - zoom map

`X` - redo zoom

`Q | E` - rotate map

`P` - change regime

`V` - change vision of regime and change display type in engine mode 

`0` – return to the center of the map

### Engine mode controls
`+` – move the sight up

`[` – move the sight left

`]` – move the sight right

`'` – move the sight down

`.` – return the sight on center of map

### Spawn objects in engine mode
`C` – set the circumference, and then select a size from 1 to 9 inclusive, using keyboard

`R` - define a rectangle and then use `Enter` to specify the points

`L` - define a line segment and then use `Enter` to specify the points

## Settings
You can change default settings in file `default_settings.json`

In the graphics column you can enable certain settings. If `multisampling` or  `depth buffering` is disabled, the corresponding parameters will not work
