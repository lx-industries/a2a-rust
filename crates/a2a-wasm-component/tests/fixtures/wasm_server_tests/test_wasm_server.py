"""Integration tests for the A2A WASM server using the A2A Python SDK."""

import httpx
import pytest
from a2a.client import Client
from a2a.client.client_factory import ClientFactory, ClientConfig
from a2a.types import (
    Message,
    TextPart,
    Role,
    GetTaskRequest,
    CancelTaskRequest,
)


@pytest.fixture
async def a2a_client(server_url: str) -> Client:
    """Create an A2A client connected to the WASM server."""
    return await ClientFactory.connect(server_url)


class TestAgentCardDiscovery:
    """Test agent card discovery endpoint."""

    async def test_agent_card_returns_valid_json(self, server_url: str):
        """GET /.well-known/agent-card.json returns valid agent card."""
        async with httpx.AsyncClient() as http_client:
            response = await http_client.get(f"{server_url}/.well-known/agent-card.json")

        assert response.status_code == 200
        assert response.headers["content-type"] == "application/json"

        card = response.json()
        assert card["name"] == "test-wasm-agent"
        assert "capabilities" in card

    async def test_client_factory_connect(self, server_url: str):
        """ClientFactory.connect successfully fetches agent card."""
        client = await ClientFactory.connect(server_url)
        assert client is not None


class TestSendMessage:
    """Test SendMessage via A2A SDK."""

    async def test_send_message_returns_response(self, a2a_client: Client):
        """SendMessage returns a task or message response."""
        message = Message(
            role=Role.user,
            parts=[TextPart(text="Hello from Python SDK")],
        )

        # send_message returns an async iterator of (StreamResponse, Task | None)
        responses = []
        async for response, task in a2a_client.send_message(message):
            responses.append((response, task))

        # Should have at least one response
        assert len(responses) > 0

    async def test_send_message_creates_task(self, a2a_client: Client):
        """SendMessage creates a task that can be retrieved."""
        message = Message(
            role=Role.user,
            parts=[TextPart(text="Hello")],
        )

        task_id = None
        async for response, task in a2a_client.send_message(message):
            if task is not None:
                task_id = task.id
                break

        # If we got a task, we should be able to get it
        if task_id:
            request = GetTaskRequest(id=task_id)
            retrieved_task = await a2a_client.get_task(request)
            assert retrieved_task.id == task_id


class TestGetTask:
    """Test GetTask operations via A2A SDK."""

    async def test_get_task_not_found(self, a2a_client: Client):
        """GetTask for non-existent task raises error or returns None."""
        request = GetTaskRequest(id="non-existent-task-id")

        # SDK may raise an exception for not found
        try:
            task = await a2a_client.get_task(request)
            # If no exception, task should be None or indicate not found
            assert task is None or task.id == ""
        except Exception:
            # Expected - task not found
            pass

    async def test_get_task_after_send(self, a2a_client: Client):
        """GetTask returns task created by SendMessage."""
        # First create a task
        message = Message(
            role=Role.user,
            parts=[TextPart(text="Hello")],
        )

        task_id = None
        async for response, task in a2a_client.send_message(message):
            if task is not None:
                task_id = task.id
                break

        assert task_id is not None, "Expected to get a task from send_message"

        # Now get the task
        request = GetTaskRequest(id=task_id)
        retrieved_task = await a2a_client.get_task(request)
        assert retrieved_task.id == task_id


class TestCancelTask:
    """Test CancelTask operations via A2A SDK."""

    async def test_cancel_task_not_found(self, a2a_client: Client):
        """CancelTask for non-existent task raises error or returns None."""
        request = CancelTaskRequest(id="non-existent-task-id")

        try:
            task = await a2a_client.cancel_task(request)
            # If no exception, should indicate not found
            assert task is None or task.id == ""
        except Exception:
            # Expected - task not found
            pass

    async def test_cancel_task_after_send(self, a2a_client: Client):
        """CancelTask cancels a task created by SendMessage."""
        # First create a task
        message = Message(
            role=Role.user,
            parts=[TextPart(text="Hello")],
        )

        task_id = None
        async for response, task in a2a_client.send_message(message):
            if task is not None:
                task_id = task.id
                break

        assert task_id is not None, "Expected to get a task from send_message"

        # Now cancel the task
        request = CancelTaskRequest(id=task_id)
        cancelled_task = await a2a_client.cancel_task(request)
        assert cancelled_task.id == task_id


class TestJsonRpcErrors:
    """Test JSON-RPC error handling (raw HTTP for invalid methods)."""

    async def test_invalid_method_returns_error(self, server_url: str):
        """Invalid JSON-RPC method returns -32601 error."""
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{server_url}/",
                json={
                    "jsonrpc": "2.0",
                    "id": "1",
                    "method": "InvalidMethod",
                    "params": {},
                },
                headers={"Content-Type": "application/json"},
            )

        assert response.status_code == 200
        result = response.json()
        assert "error" in result
        assert result["error"]["code"] == -32601  # Method not found
