# Documentation Site Delivery Checklist

- [x] mdBook and link checker install at their exact recorded versions.
- [x] The source tree builds without creating missing files.
- [x] Every published Markdown page appears exactly once in navigation.
- [x] Local links, fragments, image paths, and path case validate.
- [x] Generated runtime resources are local and resolve under `/eso-weave/`.
- [x] Search, nested navigation, edit links, and 404 output are present.
- [x] Brand, focus, reduced-motion, reflow, and contrast checks pass.
- [x] Pull requests cannot upload or deploy Pages artifacts.
- [x] Only the main-only deploy job has Pages and OIDC write permissions.
- [x] Pages uses GitHub Actions and restricts deployment to `main`.
- [x] Generated output is absent from the committed diff.
- [x] UTF-8, LF, mojibake, punctuation, and secret checks pass.
