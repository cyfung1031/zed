# gpui_combined

Facade crate that re-exports all public items from `gpui` and also exposes
`gpui_platform` as a nested module.

```rust
use gpui_combined as gpui;

let app = gpui::gpui_platform::application();
```

## Workspace setup

Add this crate to the root `Cargo.toml` workspace members:

```toml
[workspace]
members = [
    # ...
    "crates/gpui_combined",
]
```

Add it to `[workspace.dependencies]` if you want other crates to depend on it
via `workspace = true`:

```toml
gpui_combined = { path = "crates/gpui_combined" }
```

To let a downstream crate keep using the name `gpui`, depend on it like this:

```toml
gpui = { package = "gpui_combined", workspace = true }
```

or directly by path:

```toml
gpui = { package = "gpui_combined", path = "../gpui_combined" }
```
