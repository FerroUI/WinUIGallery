//! The browser host of the WinUI Gallery of FerroUI.
//!
//! The gallery is the sample `samples/WinUIGallery` of the FerroUI submodule, built from its sources.
//! The host page (`wwwroot/main.js`) creates the WebAssembly module, hands it to the script side of
//! the platform and calls [`run_main`] with the address of the page. The application class of the
//! gallery (`winui_gallery::App`) registers the XAML types, the WinUI theme and the styles of the
//! inking controls; it shows its view in a window of the desktop lifetime only, so this host makes
//! the gallery view the main view of the single-view lifetime of the browser.
//!
//! The browser has no system fonts: the embedded Inter font is the default family. The query string
//! selects the renderer (`?RenderingMode=Software2D` draws without WebGL) and the first page
//! (`?page=Button`, the identifier of a catalog entry).

#![cfg_attr(target_os = "emscripten", no_main)]

#[cfg(target_os = "emscripten")]
use wasm_bindgen::prelude::*;

/// The value of `name` in the query string of `href`.
#[cfg_attr(not(target_os = "emscripten"), allow(dead_code))]
fn query_value<'a>(href: &'a str, name: &str) -> Option<&'a str> {
    let query = href.split_once('?')?.1;
    let query = query.split('#').next().unwrap_or(query);
    query
        .split('&')
        .find_map(|pair| pair.strip_prefix(name)?.strip_prefix('='))
        .filter(|value| !value.is_empty())
}

#[cfg(target_os = "emscripten")]
#[wasm_bindgen(js_name = runMain)]
pub fn run_main(href: &str) {
    use ferroui_base::logging::LogEventLevel;
    use ferroui_base::media::FontManagerOptions;
    use ferroui_base::Ref;
    use ferroui_browser::{BrowserAppBuilder, BrowserPlatformOptions, BrowserRenderingMode};
    use ferroui_controls::{AppBuilder, Application, Control, TextBlock};
    use ferroui_fonts_inter::AppBuilderExtension;
    use ferroui_markup_xaml_winui::WinUIXamlOptions;
    use std::rc::Rc;

    let mut options = BrowserPlatformOptions::default();
    if query_value(href, "RenderingMode") == Some("Software2D") {
        options.rendering_mode = vec![BrowserRenderingMode::Software2D];
    }
    AppBuilder::configure::<winui_gallery::App>()
        .with_inter_font()
        .with(Rc::new(FontManagerOptions {
            default_family_name: Some("fonts:Inter#Inter".to_owned()),
            ..Default::default()
        }))
        .log_to_text_writer(std::io::stdout(), LogEventLevel::Warning, &[])
        .start_browser_app("out", Some(options));

    let view: Ref<Control> = match winui_gallery::GalleryView::with_preview_options(WinUIXamlOptions::for_winui_theme()) {
        Ok(gallery) => {
            if let Some(page) = query_value(href, "page") {
                gallery.navigate(page);
            }
            let root = gallery.root();
            // The handlers of the gallery hold weak references to it: it lives as long as the page.
            std::mem::forget(gallery);
            root
        }
        Err(error) => {
            let text = TextBlock::new();
            text.set_text(Some(&format!("Gallery initialization failed: {error}")));
            text.upcast()
        }
    };
    let lifetime = Application::current().and_then(|application| application.application_lifetime());
    if let Some(single_view) = lifetime.as_ref().and_then(|lifetime| lifetime.as_single_view_application_lifetime()) {
        single_view.set_main_view(Some(view));
    }
}

#[cfg(not(target_os = "emscripten"))]
fn main() {
    println!(
        "Build the browser site with: FERROUI_BROWSER_WORKSPACE=$PWD FerroUI/scripts/build-browser.sh winui-gallery-browser"
    );
}

#[cfg(test)]
mod tests {
    use super::query_value;

    #[test]
    fn the_query_string_selects_the_renderer_and_the_page() {
        let href = "https://ferroui.github.io/WinUIGallery/?RenderingMode=Software2D&page=Button#top";
        assert_eq!(query_value(href, "RenderingMode"), Some("Software2D"));
        assert_eq!(query_value(href, "page"), Some("Button"));
        assert_eq!(query_value(href, "theme"), None);
        assert_eq!(query_value("https://ferroui.github.io/WinUIGallery/", "page"), None);
    }
}
