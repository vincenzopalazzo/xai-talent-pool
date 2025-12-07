"""Collections API router."""

import logging

from fastapi import APIRouter, File, Form, HTTPException, UploadFile

from grok_service.models import (
    CollectionInfo,
    CollectionResponse,
    CreateCollectionRequest,
    DocumentInfo,
    DocumentResponse,
)
from grok_service.services import CollectionService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/collections", tags=["collections"])


@router.post("/create", response_model=CollectionResponse)
async def create_collection(request: CreateCollectionRequest) -> CollectionResponse:
    """
    Create a new collection for a talent.

    Creates a new xAI Collection that can be used to store and search
    documents related to this talent (resumes, portfolios, etc.)

    Args:
        request: Contains talent_id and talent_name

    Returns:
        CollectionResponse with the collection ID
    """
    logger.info("=" * 70)
    logger.info("COLLECTIONS API: Create collection request")
    logger.info("=" * 70)
    logger.info("Talent ID: %s", request.talent_id)
    logger.info("Talent Name: %s", request.talent_name)
    logger.info("=" * 70)

    try:
        collection_service = CollectionService()

        # Use talent_id as the unique collection name to ensure uniqueness
        collection_name = f"talent-{request.talent_id}"

        # Get existing or create new collection
        collection_id = collection_service.get_or_create_collection(collection_name)

        logger.info("=" * 70)
        logger.info("COLLECTIONS API: Collection ready")
        logger.info("Collection ID: %s", collection_id)
        logger.info("Collection Name: %s", collection_name)
        logger.info("=" * 70)

        return CollectionResponse(
            success=True,
            collection=CollectionInfo(
                collection_id=collection_id,
                collection_name=collection_name,
            ),
            error=None,
        )
    except Exception as e:
        logger.error("=" * 70)
        logger.error("COLLECTIONS API: Failed to create collection: %s", e)
        logger.error("=" * 70)
        return CollectionResponse(
            success=False,
            collection=None,
            error=str(e),
        )


@router.get("/{collection_id}", response_model=CollectionResponse)
async def get_collection(collection_id: str) -> CollectionResponse:
    """
    Get collection information by ID.

    Args:
        collection_id: The xAI collection ID

    Returns:
        CollectionResponse with collection metadata
    """
    try:
        collection_service = CollectionService()
        result = collection_service.get_collection(collection_id)

        if result is None:
            return CollectionResponse(
                success=False,
                collection=None,
                error="Collection not found",
            )

        return CollectionResponse(
            success=True,
            collection=CollectionInfo(
                collection_id=result["collection_id"],
                collection_name=result["collection_name"],
            ),
            error=None,
        )
    except Exception as e:
        logger.error("COLLECTIONS API: Failed to get collection: %s", e)
        return CollectionResponse(
            success=False,
            collection=None,
            error=str(e),
        )


@router.delete("/{collection_id}")
async def delete_collection(collection_id: str) -> dict:
    """
    Delete a collection.

    Args:
        collection_id: The xAI collection ID to delete

    Returns:
        Success status
    """
    try:
        collection_service = CollectionService()
        success = collection_service.delete_collection(collection_id)
        return {"success": success}
    except Exception as e:
        logger.error("COLLECTIONS API: Failed to delete collection: %s", e)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/documents/upload", response_model=DocumentResponse)
async def upload_document(
    collection_id: str = Form(..., description="Collection ID to upload to"),
    document_name: str = Form(..., description="Name for the document"),
    old_document_id: str | None = Form(
        None, description="Previous document ID to replace"
    ),
    document: UploadFile = File(..., description="Document file to upload"),
) -> DocumentResponse:
    """
    Upload a document to a collection.

    If old_document_id is provided, the old document will be deleted first.

    Args:
        collection_id: The collection to upload to
        document_name: Name for the document
        old_document_id: Optional previous document ID to delete
        document: The document file

    Returns:
        DocumentResponse with the new document ID
    """
    logger.info("=" * 70)
    logger.info("DOCUMENTS API: Upload document request")
    logger.info("=" * 70)
    logger.info("Collection ID: %s", collection_id)
    logger.info("Document name: %s", document_name)
    logger.info("File name: %s", document.filename)
    logger.info("Content type: %s", document.content_type)
    if old_document_id:
        logger.info("Replacing document: %s", old_document_id)
    logger.info("=" * 70)

    try:
        # Read the document content
        content = await document.read()
        logger.info("Document size: %d bytes", len(content))

        collection_service = CollectionService()

        # Upload document (this also handles deletion of old document)
        document_id = collection_service.upload_document(
            collection_id=collection_id,
            name=document_name,
            data=content,
            old_document_id=old_document_id,
        )

        logger.info("=" * 70)
        logger.info("DOCUMENTS API: Upload successful")
        logger.info("New document ID: %s", document_id)
        logger.info("=" * 70)

        return DocumentResponse(
            success=True,
            document=DocumentInfo(
                document_id=document_id,
                document_name=document_name,
            ),
            error=None,
        )
    except Exception as e:
        logger.error("=" * 70)
        logger.error("DOCUMENTS API: Upload failed: %s", e)
        logger.error("=" * 70)
        return DocumentResponse(
            success=False,
            document=None,
            error=str(e),
        )


@router.delete("/{collection_id}/documents/{document_id}")
async def remove_document(collection_id: str, document_id: str) -> dict:
    """
    Remove a document from a collection.

    Args:
        collection_id: The collection ID
        document_id: The document ID to remove

    Returns:
        Success status
    """
    try:
        collection_service = CollectionService()
        success = collection_service.remove_document(collection_id, document_id)
        return {"success": success}
    except Exception as e:
        logger.error("DOCUMENTS API: Failed to remove document: %s", e)
        raise HTTPException(status_code=500, detail=str(e))
