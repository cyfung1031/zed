# gpui_facade

A facade crate that re-exports `gpui`, exposes `gpui_platform`, and can optionally expose related GPUI packages.

## Example

```toml
gpui = { package = "gpui_facade", workspace = true }
```

```rust
use gpui::*;

let app = gpui::gpui_platform::application();
```

## Optional package re-exports

```toml
gpui = { package = "gpui_facade", workspace = true, features = ["gpui_web"] }
```

```rust
use gpui::*;

#[cfg(target_family = "wasm")]
let _ = gpui::gpui_web::WebPlatform::new(true);
```

## Features

```toml
default = ["font-kit", "wayland", "x11", "windows-manifest"]

test-support = [
    "gpui/test-support",
    "gpui_platform/test-support",
    "gpui_macos?/test-support",
    "gpui_linux?/test-support",
    "gpui_windows?/test-support",
]

bench = ["gpui/bench"]
inspector = ["gpui/inspector", "gpui_macros/inspector"]
leak-detection = ["gpui/leak-detection"]

wayland = ["gpui/wayland", "gpui_platform/wayland", "gpui_linux?/wayland"]
x11 = ["gpui/x11", "gpui_platform/x11", "gpui_linux?/x11"]

screen-capture = [
    "gpui/screen-capture",
    "gpui_platform/screen-capture",
    "gpui_macos?/screen-capture",
    "gpui_linux?/screen-capture",
    "gpui_windows?/screen-capture",
]

windows-manifest = ["gpui/windows-manifest"]
input-latency-histogram = ["gpui/input-latency-histogram"]
profiler = ["gpui/profiler"]

font-kit = ["gpui_platform/font-kit", "gpui_macos?/font-kit"]
runtime_shaders = ["gpui_platform/runtime_shaders", "gpui_macos?/runtime_shaders"]

gpui_macros = ["dep:gpui_macros"]
gpui_web = ["dep:gpui_web"]
gpui_web_multithreaded = ["gpui_web", "gpui_web/multithreaded"]

gpui_macos = ["dep:gpui_macos"]
gpui_linux = ["dep:gpui_linux"]
gpui_windows = ["dep:gpui_windows"]
```
