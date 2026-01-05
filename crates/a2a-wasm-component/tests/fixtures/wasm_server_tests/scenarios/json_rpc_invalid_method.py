"""Test JSON-RPC invalid method returns error."""

import asyncio
import json
import os

import httpx


async def main():
    server_url = os.environ["WASM_SERVER_URL"]

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

    result = response.json()

    print(json.dumps({
        "step": "invalid_method",
        "status_code": response.status_code,
        "has_error": "error" in result,
        "error_code": result.get("error", {}).get("code"),
    }))


if __name__ == "__main__":
    asyncio.run(main())
