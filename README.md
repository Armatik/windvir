# WINDVIR
The simplest 2D simulation of wind currents

For compile and run project you must use command on UNIX-like
```
RUST_LOG=windvir cargo run --release
```
On Windows
```
cargo run --release
```
If you want to use `FFI` protocol, then you need compile with command on UNIX-like
```
RUST_LOG=windvir cargo run --release -- -c
```
On Windows
```
cargo run --release -- -c
```
If you want to start application with rainbow field, then you need compile with command on UNIX-like
```
RUST_LOG=windvir cargo run --release -- -r
```
On Windows
```
cargo run --release -- -r
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

`F` - define a polygon and then use `Enter` to specify the poits

## Settings
You can change default settings in file `default_settings.json`

In the graphics column you can enable certain settings. If `multisampling` or  `depth buffering` is disabled, the corresponding parameters will not work

## Shapes at launch

In order for the figures to be immediately spawned at start. Configure file `data/figures.json`.

Example of json design:
```json
{
    "circles": [
        {
            "x": 0.0,
            "y": 0.0,
            "radius": 0.0,
            "is_fill": true,
            "rgb": [0.0, 0.0, 0.0]
        }
    ],
    "rectangles": [
        {
            "left_up_angle_x": 0.0,
            "left_up_angle_y": 0.0,
            "right_down_angle_x": 0.0,
            "right_down_angle_y": 0.0,
            "is_fill": true,
            "rgb": [0.0, 0.0, 0.0]
        }
    ],
    "lines": [
        {
            "p0_x": 0.0,
            "p0_y": 0.0,
            "p1_x": 0.0,
            "p1_y": 0.0,
            "rgb": [0.0, 0.0, 0.0]
        }
    ],
    "polygons": [
        {
            "points": [[0.0, 0.0]],
            "is_fill": true,
            "rgb": [0.0, 0.0, 0.0]
        }
    ]
}
```
