// src/assets/styles.rs

pub struct StylesContent;

impl StylesContent {
    pub fn css_files() -> &'static str {
        concat!(
            "# CSS File Management\n\n",
            "## Including CSS Files\n\n",
            "Use the document::Stylesheet component to include CSS files in your Dioxus app.\n",
            "Reference CSS files from your assets folder using the asset! macro.\n\n",
            "## Project Structure\n\n",
            "Organize your styles:\n",
            "- Global styles in main CSS file\n",
            "- Component-specific styles in separate files\n",
            "- Theme files for dark/light mode\n\n",
            "## Inline Styles\n\n",
            "You can also apply styles directly to elements using the style attribute.\n"
        )
    }

    pub fn themes() -> &'static str {
        concat!(
            "# Theme System\n\n",
            "## Creating Themes\n\n",
            "Use CSS custom properties (variables) for easy theming.\n",
            "Define color variables in :root and override them in theme-specific selectors.\n\n",
            "Common theme variables:\n",
            "- Background colors (primary, secondary)\n",
            "- Text colors (primary, secondary)\n",
            "- Accent colors\n",
            "- Border colors\n\n",
            "## Applying Themes\n\n",
            "Set a data-theme attribute on a root element to switch themes.\n",
            "Use Dioxus signals to manage theme state.\n\n",
            "## Dynamic Theme Switching\n\n",
            "Create a toggle function that switches between theme values.\n",
            "Store theme preference in browser storage for persistence.\n"
        )
    }
}