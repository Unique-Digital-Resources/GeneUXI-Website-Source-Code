// src/assets/icons.rs

pub struct IconsContent;

impl IconsContent {
    pub fn material_design() -> &'static str {
        concat!(
            "# Material Design Icons (MDI)\n\n",
            "## Setup\n\n",
            "Include the MDI font in your app by adding the stylesheet link.\n\n",
            "## Usage\n\n",
            "Use MDI icons with the 'mdi' class prefix:\n",
            "- mdi-home - Home icon\n",
            "- mdi-menu - Menu/hamburger icon\n",
            "- mdi-close - Close/X icon\n",
            "- mdi-account - User profile icon\n",
            "- mdi-settings - Settings gear icon\n",
            "- mdi-search - Search magnifying glass\n",
            "- mdi-heart - Heart/favorite icon\n",
            "- mdi-star - Star/rating icon\n",
            "- mdi-check - Checkmark icon\n",
            "- mdi-alert - Alert/warning icon\n\n",
            "Browse all icons at: https://pictogrammers.com/library/mdi/\n"
        )
    }

    pub fn custom_icons() -> &'static str {
        concat!(
            "# Custom Icons\n\n",
            "## SVG Icons\n\n",
            "You can include SVG icons directly in your components as inline SVG elements.\n\n",
            "## Icon Components\n\n",
            "Create reusable icon components by wrapping SVG elements in Dioxus components.\n\n",
            "## Icon Sprites\n\n",
            "Use SVG sprites for better performance by defining symbols once and referencing them multiple times.\n\n",
            "## Loading Icon Fonts\n\n",
            "For custom icon fonts, add the font-face definition to your CSS:\n",
            "- Define the font family\n",
            "- Specify the source file path\n",
            "- Apply the font to icon elements\n"
        )
    }
}