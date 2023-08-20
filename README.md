<div align="center">

# {{project-name}}

[![crates.io](https://img.shields.io/crates/v/{{crate_name}}.svg)](https://crates.io/crates/{{crate_name}})
[![docs.rs](https://docs.rs/{{crate_name}}/badge.svg)](https://docs.rs/{{crate_name}})
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
{{crate_name}} = { version = "0.0.1" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use {{crate_name}}::MyPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(MyPlugin::new())
    .build()
    .unwrap();

```
