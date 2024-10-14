> Fork of wyhaya's tauri-plugin-theme, supporting changing title bar's color for Windows

# tauri-plugin-theme

Tauri currently doesn't support dynamically change app theme, this plugin makes up for that.

https://github.com/wyhaya/tauri-plugin-theme/assets/23690145/2422ce95-418d-4f07-adf5-e78af2552f51

> [!NOTE]  
> This only applies to the `Tauri 2x` version. If you are using the Tauri 1x version, please refer
> to [tauri-1x-version](https://github.com/wyhaya/tauri-plugin-theme/tree/tauri-1x-version).

## Install

```bash
cargo add tauri-plugin-theme@2.1.2
```

```rust
let mut ctx = tauri::generate_context!();
tauri::Builder::default ()
// Init plugin and auto restore window theme !!!
.plugin(tauri_plugin_theme::init(ctx.config_mut()))
// ...
.run(ctx)
// ...
```

### Tauri permissions

Add to `src-tauri/capabilities/default.json`

```json5
{
  ...
  "permissions": [
    ...
    "theme:default"
  ]
}
```

## Usage

```javascript
// Follow system theme setting
invoke("plugin:theme|set_theme", {
  theme: "auto",
});

// Always use light theme
invoke("plugin:theme|set_theme", {
  theme: "light",
});

// Always use dark theme
invoke("plugin:theme|set_theme", {
  theme: "dark",
});

// Set title bar color (Windows Only)
invoke("plugin:theme|set_color", {
  color: 0xff6a6a,
});

// Get theme (default: auto)
const theme = await invoke("plugin:theme|get_theme");

// Get title bar color (default: 0x000000, Windows Only)
const color = await invoke("plugin:theme|get_color");
```

## Support

| MacOS | Linux | Windows | Android | iOS |
|-------|-------|---------|---------|-----|
| ✅     | ✅     | ✅       | ❌       | ❌   |

## NOTE

### For Windows

Requires WebView2 Runtime version 101.0.1210.39(May 9, 2022) or higher; otherwise, the app will complete the theme
change by `restart`.

See: [https://learn.microsoft.com/en-us/microsoft-edge/webview2/release-notes/archive?tabs=dotnetcsharp#10121039](https://learn.microsoft.com/en-us/microsoft-edge/webview2/release-notes/archive?tabs=dotnetcsharp#10121039)

### For Linux

On Linux platform, it has not been extensively tested.
