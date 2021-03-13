// Licensed under the MIT License <https://opensource.org/licenses/MIT>
use super::{ChangeLog, Result};
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

type RenderResult = ::std::result::Result<(), RenderError>;

pub fn render_template(template: &str, clog: &ChangeLog) -> Result<String> {
    let mut hbs = Handlebars::new();
    hbs.register_helper("tidy-change", Box::new(tidy));
    hbs.render_template(template, clog)
        .map_err(|e| format_err!("Handlebar render failed: {}", e))
}

/// A handlebar helper to tidy up markdown lists used to render changes.
fn tidy(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> RenderResult {
    if let Some(indent) = h.param(0).and_then(|v| v.value().as_str()) {
        if let Some(text) = h.param(1).and_then(|v| v.value().as_str()) {
            let mut lines = text.lines();
            if let Some(first) = lines.next() {
                out.write(first.trim())?;
                out.write("\n")?;
            }
            for line in lines {
                out.write(indent)?;
                out.write(line)?;
                out.write("\n")?;
            }
        }
    }
    Ok(())
}
