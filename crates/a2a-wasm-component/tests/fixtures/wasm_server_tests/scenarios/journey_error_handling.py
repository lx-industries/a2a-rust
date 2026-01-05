"""Test error handling: various error conditions."""

import asyncio
import json
import os

import httpx
from a2a.client.client_factory import ClientFactory
from a2a.types import GetTaskRequest, CancelTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    # Error 1: Get non-existent task
    try:
        await client.get_task(GetTaskRequest(name="nonexistent-1"))
        print(json.dumps({"step": "get_nonexistent", "error": False}))
    except Exception as e:
        print(json.dumps({"step": "get_nonexistent", "error": True, "error_type": type(e).__name__}))

    # Error 2: Cancel non-existent task
    try:
        await client.cancel_task(CancelTaskRequest(name="nonexistent-2"))
        print(json.dumps({"step": "cancel_nonexistent", "error": False}))
    except Exception as e:
        print(json.dumps({"step": "cancel_nonexistent", "error": True, "error_type": type(e).__name__}))

    # Error 3: Invalid JSON-RPC method
    async with httpx.AsyncClient() as http_client:
        response = await http_client.post(
            f"{server_url}/",
            json={"jsonrpc": "2.0", "id": "1", "method": "BadMethod", "params": {}},
            headers={"Content-Type": "application/json"},
        )
    result = response.json()
    print(json.dumps({
        "step": "invalid_method",
        "has_error": "error" in result,
        "error_code": result.get("error", {}).get("code"),
    }))


if __name__ == "__main__":
    asyncio.run(main())
