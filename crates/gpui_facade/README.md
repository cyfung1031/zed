# gpui_facade

A small facade crate that re-exports `gpui`, `gpui_macros`, `gpui_web`, and exposes `gpui_platform` under `gpui_facade::gpui_platform`.

This avoids making the real `gpui` crate depend on `gpui_platform`, so it does not introduce a dependency cycle.

Example:

```rust
use gpui_facade as gpui;

let app = gpui::gpui_platform::application();
```

You can also depend on it under the crate name `gpui` from another crate:

```toml
gpui = { package = "gpui_facade", workspace = true }
```
