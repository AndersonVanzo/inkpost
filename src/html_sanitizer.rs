use anyhow::Context;
use lol_html::{RewriteStrSettings, element, rewrite_str};

pub fn strip_tags(input: &str) -> anyhow::Result<String> {
    let output = rewrite_str(
        input,
        RewriteStrSettings::new().append_element_content_handler(element!(
            "div.highlight__panel",
            |el| {
                el.remove();
                Ok(())
            }
        )),
    )
    .context("failed to parse article content");

    output
}

#[cfg(test)]
#[test]
fn strips_div_highlight_panel() {
    let html = r#"<pre><div class="highlight__panel"><p>this should be removed</p></div></pre>"#;
    let output = strip_tags(&html).expect("strip_tags should pass");
    assert_eq!(output, "<pre></pre>");
}
