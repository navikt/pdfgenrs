# Accessibility

`pdfgenrs` is an API for generating documents, not a user-facing web application.
Accessibility depends on both the renderer and the templates, content, and
application used to deliver the generated documents.

## PDF accessibility support

The Typst PDF renderer is configured to export PDF/A-2a and PDF/UA-1 together.
PDF/A addresses archival requirements; PDF/UA addresses accessible PDF structure.
Export errors are returned when Typst cannot export a document with these settings.

Successful export is not a guarantee that a document is usable with assistive
technology or meets all applicable accessibility requirements. Meaningful
alternative text, logical reading order, clear language, and visual presentation
still require authoring and review.

| Generation path | Support and limitations |
|-----------------|-------------------------|
| Typst + JSON | Uses the PDF/A-2a and PDF/UA-1 export settings. Template authors control document structure, language, title, and alternative text. |
| HTML to PDF | Uses a separate HTML converter. The Typst renderer's PDF/A-2a and PDF/UA-1 settings are not applied by this path; validate its output separately. |
| Image to PDF | Uses the Typst PDF export settings, but supplies only the generic alternative text `Uploaded image` and title `Image`. It does not perform OCR or provide a meaningful description of image content. |
| Typst + JSON to HTML | Produces HTML rather than PDF. Review the exported HTML and the application presenting it separately. |

This document describes implementation support and authoring guidance, not a
certification of WCAG or PDF/UA conformance for every generated document.

## Authoring accessible templates

- Set a meaningful document title and the correct text language in the template.
  Use `nb` for Norwegian Bokmal, `nn` for Norwegian Nynorsk, or `en` for English,
  as appropriate. Mark passages in another language with their own text language.
- Use semantic headings with a logical hierarchy, real lists, and table headers
  rather than simulating structure with bold text, spacing, or layout grids.
- Keep reading order logical, especially in columns, grids, and positioned content.
  Visual placement alone does not establish a useful screen-reader reading order.
- Provide meaningful alternative text for informative images. Keep decorative
  content out of the assistive reading flow using Typst's supported mechanisms.
- Use descriptive link text and do not convey information through color alone.
  Check text and non-text contrast against the applicable WCAG requirements.
- Use readable text sizes and layouts that remain usable when magnified. Keep
  essential information as text rather than embedding it only in an image.
- Review dynamic content, including long text, missing optional fields, tables
  spanning pages, and mixed-language documents.

For example, a Norwegian Bokmal template can start with:

```typst
#set document(title: "Brev fra Nav")
#set text(lang: "nb")

= Kontakt oss

Kontakt oss for mer informasjon om brevet.
```

See [the Nav letter template](templates/nav-brev/brev.typ) for an existing example
with a document title, language, headings, and logo alternative text. Set language
in the Typst source; the PDF endpoints do not expose a `language` request header.

For image-based source material, prefer a structured template containing the
equivalent text and meaningful image descriptions over the image-to-PDF endpoint.

## Checking generated documents

Check representative generated documents before release and after changes to
templates or rendering dependencies:

1. Run a PDF accessibility validator that supports PDF/UA-1. Review its findings;
   automated validation cannot assess whether descriptions and reading order make
   sense to a reader.
2. Inspect the document title, language, tags, headings, table structure, links,
   alternative text, and reading order.
3. Read the document with a screen reader in a PDF reader that supports tagged
   PDFs. Check navigation, pronunciation, image descriptions, and table reading.
4. Check keyboard access to links, text selection, zoom, and visual contrast.
5. Repeat with realistic content variations, not only a minimal sample.

Validate HTML output separately for semantic structure, keyboard interaction, and
screen-reader behavior in the application that presents it.

## Reporting accessibility issues

Report accessibility problems through this repository's GitHub issues, following
the [contact guidance](README.md#contact). Include the generation endpoint,
`pdfgenrs` version, expected and actual behavior, and the relevant PDF reader,
screen reader, or validator and its version.

Provide a minimal template and synthetic input that reproduce the problem when
possible. Do not attach personal data, confidential documents, or sensitive logs.
For security vulnerabilities, follow [SECURITY.md](SECURITY.md) instead.
