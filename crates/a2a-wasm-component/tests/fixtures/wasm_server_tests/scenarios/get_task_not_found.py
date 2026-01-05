"""Test GetTask for non-existent task returns error."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import GetTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    request = GetTaskRequest(name="non-existent-task-id")

    try:
        task = await client.get_task(request)
        print(json.dumps({
            "step": "get_task_not_found",
            "error": False,
            "task_is_none": task is None,
        }))
    except Exception as e:
        print(json.dumps({
            "step": "get_task_not_found",
            "error": True,
            "error_type": type(e).__name__,
        }))


if __name__ == "__main__":
    asyncio.run(main())
