"""Test GetTask returns task created by SendMessage."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role, GetTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    # First create a task
    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello")],
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

    # Now get the task
    request = GetTaskRequest(name=task_id)
    retrieved_task = await client.get_task(request)

    print(json.dumps({
        "step": "get_task",
        "task_id_matches": retrieved_task.id == task_id,
    }))


if __name__ == "__main__":
    asyncio.run(main())
