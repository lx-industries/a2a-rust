"""Test basic flow: send message -> get task -> cancel task."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role, GetTaskRequest, CancelTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    # Step 1: Send message
    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello from journey test")],
    )

    task_id = None
    async for response, task in client.send_message(message):
        if task is not None:
            task_id = task.id
            break

    print(json.dumps({
        "step": "send_message",
        "got_task_id": task_id is not None,
    }))

    assert task_id is not None, "Expected to get a task from send_message"

    # Step 2: Get task
    get_request = GetTaskRequest(name=task_id)
    retrieved_task = await client.get_task(get_request)

    print(json.dumps({
        "step": "get_task",
        "task_id_matches": retrieved_task.id == task_id,
        "has_history": len(retrieved_task.history) > 0 if retrieved_task.history else False,
    }))

    # Step 3: Cancel task
    cancel_request = CancelTaskRequest(name=task_id)
    cancelled_task = await client.cancel_task(cancel_request)

    print(json.dumps({
        "step": "cancel_task",
        "task_id_matches": cancelled_task.id == task_id,
    }))


if __name__ == "__main__":
    asyncio.run(main())
