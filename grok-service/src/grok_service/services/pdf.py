"""PDF text extraction service."""

import io

from pypdf import PdfReader


def extract_text_from_pdf(pdf_content: bytes) -> str:
    """
    Extract text content from a PDF file.

    Args:
        pdf_content: Raw bytes of the PDF file

    Returns:
        Extracted text content from all pages
    """
    pdf_file = io.BytesIO(pdf_content)
    reader = PdfReader(pdf_file)

    text_parts = []
    for page in reader.pages:
        text = page.extract_text()
        if text:
            text_parts.append(text)

    return "\n\n".join(text_parts)
