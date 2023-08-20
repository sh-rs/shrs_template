use shrs::prelude::*;
use {{crate_name}}::MyPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(MyPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
