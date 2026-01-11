mod config;
mod loader;

pub use config::{Conditional, TemplateConfig, TemplateSource};
pub use loader::{get_available_templates, load_template_config};
