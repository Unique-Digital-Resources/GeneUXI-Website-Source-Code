// src/assets/images.rs

pub struct ImagesContent;

impl ImagesContent {
    pub fn supported_formats() -> &'static str {
        concat!(
            "# Supported Image Formats\n\n",
            "## Raster Formats\n",
            "- PNG: Lossless compression, supports transparency\n",
            "- JPEG/JPG: Lossy compression, best for photos\n",
            "- WebP: Modern format with superior compression\n",
            "- GIF: Supports animation, limited colors\n\n",
            "## Vector Formats\n",
            "- SVG: Scalable vector graphics, perfect for icons and logos\n",
            "- PDF: For document-based graphics\n"
        )
    }

    pub fn optimization() -> &'static str {
        concat!(
            "# Image Optimization\n\n",
            "## Best Practices\n\n",
            "1. Choose the right format:\n",
            "   - Photos: JPEG or WebP\n",
            "   - Graphics with transparency: PNG or WebP\n",
            "   - Icons and logos: SVG\n\n",
            "2. Compress images:\n",
            "   - Use tools like imagemagick, squoosh, or tinypng\n",
            "   - Aim for under 200KB for photos, under 50KB for graphics\n\n",
            "3. Use responsive images with srcset attributes\n\n",
            "4. Enable lazy loading for below-the-fold images\n\n",
            "## Performance Tips\n",
            "- Serve images in next-gen formats (WebP, AVIF)\n",
            "- Use CDN for faster delivery\n",
            "- Consider using image sprites for small icons\n"
        )
    }
}