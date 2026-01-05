"""Test agent card discovery endpoint."""

import asyncio
import json
import os

import httpx
from a2a.client.client_factory import ClientFactory


async def main():
    server_url = os.environ["WASM_SERVER_URL"]

    # Test 1: GET agent card returns valid JSON
    async with httpx.AsyncClient() as http_client:
        response = await http_client.get(f"{server_url}/.well-known/agent-card.json")

    print(json.dumps({
        "step": "get_agent_card",
        "status_code": response.status_code,
        "content_type": response.headers["content-type"],
        "name": response.json().get("name"),
        "has_capabilities": "capabilities" in response.json(),
    }))

    # Test 2: ClientFactory.connect works
    client = await ClientFactory.connect(server_url)
    print(json.dumps({
        "step": "client_factory_connect",
        "success": client is not None,
    }))


if __name__ == "__main__":
    asyncio.run(main())
