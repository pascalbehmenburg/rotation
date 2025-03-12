use gloo_storage::{LocalStorage, Storage};

use dioxus::prelude::*;

#[derive(Clone)]
pub struct ThemeState {
    _theme: Signal<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    children: Element,
    #[props(default = String::from("system"))]
    default_theme: Option<String>,
    #[props(default = String::from("theme"))]
    storage_key: Option<String>,
}

/// Theme provider component which sets the theme class on the document root
/// and provides the theme state to the context while ensuring that the context is a singleton.
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    // ignore nested context providers, just passtrough children
    match try_consume_context::<ThemeState>() {
        Some(_) => props.children,
        None => rsx!(Theme { ..props }),
    }
}

/// Retrieves the theme from local storage or returns the fallback
fn get_theme(key: impl Into<String>, fallback: impl Into<String>) -> String {
    let theme: String = LocalStorage::get(key.into()).unwrap_or(fallback.into());
    theme
}

/// Retrieves `prefers-color-scheme` media query from the browser and returns dark or light
fn get_system_theme() -> String {
    let window = web_sys::window().unwrap();
    let prefers_dark_colors = window
        .match_media("(prefers-color-scheme: dark)")
        .unwrap_or(None)
        .is_some_and(|v| v.matches());
    if prefers_dark_colors {
        "dark".to_string()
    } else {
        "light".to_string()
    }
}

/// Inner context provider which sets the theme class on the document root
#[component]
fn Theme(props: ThemeProviderProps) -> Element {
    let theme = get_theme(props.storage_key.unwrap(), props.default_theme.unwrap());
    let theme = use_signal(|| theme);

    use_effect(move || {
        let document_root = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .document_element()
            .unwrap();

        // Remove existing theme classes
        let base_class = document_root
            .class_name()
            .as_str()
            .replace("light", "")
            .replace("dark", "")
            .trim()
            .to_string();

        // Determine the theme to apply
        let applied_theme = if theme() == "system" {
            get_system_theme()
        } else {
            theme()
        };

        // Set the class with the theme
        let new_class = if base_class.is_empty() {
            applied_theme
        } else {
            format!("{} {}", base_class, applied_theme)
        };

        document_root.set_class_name(&new_class);
    });

    use_context_provider(|| ThemeState { _theme: theme });

    rsx! {
        {props.children}
    }
}
