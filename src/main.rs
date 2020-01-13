use pbgui_style_engine::{render_template, TreeStyle};
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let treestyle = TreeStyle::default();
    println!("{}", serde_json::to_string_pretty(&treestyle)?);

    let result = render_template("tree", "./templates/tree.qss", &treestyle)?;
    println!("STYLE SHEET");
    println!("{}", result);
    Ok(())
}
