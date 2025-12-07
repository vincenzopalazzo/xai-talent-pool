"""Collection service for xAI Collections API."""

import logging
from typing import Optional

from xai_sdk import Client

from grok_service.config import get_settings

logger = logging.getLogger(__name__)


class CollectionService:
    """Service for managing xAI Collections."""

    def __init__(self) -> None:
        """Initialize the Collection service."""
        self.settings = get_settings()
        self.client = Client()

    def create_collection(self, name: str) -> str:
        """
        Create a new collection for a talent.

        Args:
            name: The name for the collection (typically talent name or ID)

        Returns:
            The collection ID

        Raises:
            Exception: If collection creation fails
        """
        logger.info("=" * 70)
        logger.info("COLLECTIONS: Creating collection for: %s", name)
        logger.info("=" * 70)

        try:
            result = self.client.collections.create(name=name)
            collection_id = result.collection_id

            logger.info("=" * 70)
            logger.info("COLLECTIONS: Collection created successfully")
            logger.info("Collection ID: %s", collection_id)
            logger.info("Collection Name: %s", result.collection_name)
            logger.info("=" * 70)

            return collection_id
        except Exception as e:
            logger.error("=" * 70)
            logger.error("COLLECTIONS: Failed to create collection: %s", e)
            logger.error("=" * 70)
            raise

    def get_collection(self, collection_id: str) -> Optional[dict]:
        """
        Get collection metadata by ID.

        Args:
            collection_id: The collection ID

        Returns:
            Collection metadata or None if not found
        """
        try:
            result = self.client.collections.get(collection_id=collection_id)
            return {
                "collection_id": result.collection_id,
                "collection_name": result.collection_name,
                "documents_count": result.documents_count,
            }
        except Exception as e:
            logger.error(
                "COLLECTIONS: Failed to get collection %s: %s", collection_id, e
            )
            return None

    def list_collections(self, filter_name: Optional[str] = None) -> list[dict]:
        """
        List all collections, optionally filtered by name.

        Args:
            filter_name: Optional name filter

        Returns:
            List of collection metadata
        """
        try:
            filter_expr = f'collection_name:"{filter_name}"' if filter_name else None
            result = self.client.collections.list(filter=filter_expr)
            return [
                {
                    "collection_id": c.collection_id,
                    "collection_name": c.collection_name,
                    "documents_count": c.documents_count,
                }
                for c in result.collections
            ]
        except Exception as e:
            logger.error("COLLECTIONS: Failed to list collections: %s", e)
            return []

    def find_collection_by_name(self, name: str) -> Optional[str]:
        """
        Find a collection by exact name match.

        Args:
            name: The collection name to find

        Returns:
            The collection ID if found, None otherwise
        """
        collections = self.list_collections(filter_name=name)
        for c in collections:
            if c["collection_name"] == name:
                return c["collection_id"]
        return None

    def get_or_create_collection(self, name: str) -> str:
        """
        Get an existing collection by name or create a new one.

        Args:
            name: The collection name

        Returns:
            The collection ID
        """
        existing_id = self.find_collection_by_name(name)
        if existing_id:
            logger.info(
                "COLLECTIONS: Found existing collection %s for %s", existing_id, name
            )
            return existing_id

        return self.create_collection(name)

    def delete_collection(self, collection_id: str) -> bool:
        """
        Delete a collection.

        Args:
            collection_id: The collection ID to delete

        Returns:
            True if deleted successfully
        """
        try:
            self.client.collections.delete(collection_id=collection_id)
            logger.info("COLLECTIONS: Deleted collection %s", collection_id)
            return True
        except Exception as e:
            logger.error(
                "COLLECTIONS: Failed to delete collection %s: %s", collection_id, e
            )
            return False

    def upload_document(
        self,
        collection_id: str,
        name: str,
        data: bytes,
        old_document_id: str | None = None,
    ) -> str:
        """
        Upload a document to a collection.

        If old_document_id is provided, it will be deleted first.

        Args:
            collection_id: The collection ID to upload to
            name: The document name
            data: The document content as bytes
            old_document_id: Optional previous document ID to delete

        Returns:
            The new document ID

        Raises:
            Exception: If upload fails
        """
        logger.info("=" * 70)
        logger.info("DOCUMENTS: Uploading document to collection %s", collection_id)
        logger.info("Document name: %s", name)
        logger.info("Document size: %d bytes", len(data))
        if old_document_id:
            logger.info("Old document to replace: %s", old_document_id)
        logger.info("=" * 70)

        # Delete old document if provided
        if old_document_id:
            try:
                self.remove_document(collection_id, old_document_id)
                logger.info("DOCUMENTS: Removed old document %s", old_document_id)
            except Exception as e:
                logger.warning(
                    "DOCUMENTS: Failed to remove old document %s: %s",
                    old_document_id,
                    e,
                )

        # Upload new document
        try:
            result = self.client.collections.upload_document(
                collection_id=collection_id,
                name=name,
                data=data,
                wait_for_indexing=False,
            )

            # The result is DocumentMetadata with file_metadata.file_id
            document_id = result.file_metadata.file_id

            logger.info("=" * 70)
            logger.info("DOCUMENTS: Upload successful")
            logger.info("Document ID: %s", document_id)
            logger.info("=" * 70)

            return document_id
        except Exception as e:
            logger.error("=" * 70)
            logger.error("DOCUMENTS: Failed to upload document: %s", e)
            logger.error("=" * 70)
            raise

    def remove_document(self, collection_id: str, document_id: str) -> bool:
        """
        Remove a document from a collection.

        Args:
            collection_id: The collection ID
            document_id: The document ID to remove

        Returns:
            True if removed successfully
        """
        try:
            self.client.collections.remove_document(
                collection_id=collection_id,
                file_id=document_id,
            )
            logger.info(
                "DOCUMENTS: Removed document %s from collection %s",
                document_id,
                collection_id,
            )
            return True
        except Exception as e:
            logger.error("DOCUMENTS: Failed to remove document %s: %s", document_id, e)
            return False
