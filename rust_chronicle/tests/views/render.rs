use fluent_templates::{ArcLoader, FluentLoader};
use loco_rs::controller::views::{engines, ViewRenderer};

/// Renders the shipped Tera view through the same view engine the app builds at
/// boot, including the i18n `t()` function the template calls.
#[test]
fn renders_home_view_with_i18n() {
    let loader = std::sync::Arc::new(
        ArcLoader::builder("assets/i18n", unic_langid::langid!("en-US"))
            .shared_resources(Some(&["assets/shared.ftl".into()]))
            .customize(|bundle| bundle.set_use_isolating(false))
            .build()
            .expect("locales should load"),
    );

    let view = engines::TeraView::build_with_post_process(move |tera| {
        tera.register_function("t", FluentLoader::new(loader.clone()));
        Ok(())
    })
    .expect("view engine should build");

    let rendered_broadsheet = view
        .render("home/index.html", serde_json::json!({}))
        .expect("broadsheet home view should render");

    assert!(
        rendered_broadsheet.contains("The Rust Chronicle"),
        "expected broadsheet title, got: {rendered_broadsheet}"
    );
}
