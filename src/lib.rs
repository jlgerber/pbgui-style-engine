use handlebars::Handlebars;
use serde_derive::{Deserialize, Serialize};
use std::default::Default;
pub mod stylesheet;
pub use stylesheet::{Border, Color, Size, Size4};

#[derive(Debug, Serialize)]
pub struct PackageTreeView {
    background: Color,
    sel_bg_color: Color,
}

impl Default for PackageTreeView {
    fn default() -> Self {
        Self {
            //"rgb(40,40,40)".to_string(),
            background: Color::Rgb {
                r: 30,
                g: 40,
                b: 40,
            },
            //"rgb(40,40,40)".to_string(),
            sel_bg_color: Color::Rgb {
                r: 30,
                g: 40,
                b: 40,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SitesCbFrame {
    border: Border,
    background: Color,
}

impl Default for SitesCbFrame {
    fn default() -> Self {
        Self {
            border: Border::Solid {
                size: Size::Px(1),
                //"1px solid rgb(130,130,130)".to_string(),
                color: Color::Rgb {
                    r: 130,
                    g: 130,
                    b: 130,
                },
            },
            //"rgb(40,40,40)".to_string(),
            background: Color::Rgb {
                r: 40,
                g: 40,
                b: 40,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SiteLabel {
    border: Border,
}

impl Default for SiteLabel {
    fn default() -> Self {
        Self {
            border: Border::Solid {
                size: Size::Px(1),
                //"1px solid rgb(130,130,130)".to_string(),
                color: Color::Rgb {
                    r: 130,
                    g: 130,
                    b: 130,
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SiteComboBox {
    border_right: Border,
    border_radius: Size,
    padding: Size4,
    height: Size,
    padding_left: Size,
    background: Color,
}

impl Default for SiteComboBox {
    fn default() -> Self {
        Self {
            border_right: Border::Solid {
                size: Size::Px(1),
                //"1px solid rgb(130,130,130)".to_string(),
                color: Color::Rgb {
                    r: 130,
                    g: 130,
                    b: 130,
                },
            },
            border_radius: Size::Px(3),
            //"1px 18px 1px 3px".to_string(),
            padding: Size4::Px(1, 18, 1, 3),
            //"30px".to_string(),
            height: Size::Px(30),
            //"20ps".to_string(),
            padding_left: Size::Px(20),
            //"rgb(40,40,40)".to_string(),
            background: Color::Rgb {
                r: 40,
                g: 40,
                b: 40,
            },
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct TreeStyle {
    package_tree_view: PackageTreeView,
    sites_cb_frame: SitesCbFrame,
    site_label: SiteLabel,
    site_combo_box: SiteComboBox,
}

pub fn render_template<T>(
    name: &str,
    path: &str,
    data: &T,
) -> Result<String, Box<dyn std::error::Error>>
where
    T: serde::ser::Serialize,
{
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file(name, path)?;
    Ok(handlebars.render("tree", data)?)
}
