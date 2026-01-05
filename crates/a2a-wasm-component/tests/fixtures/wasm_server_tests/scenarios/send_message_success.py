"""Test SendMessage returns a response."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello from Python SDK")],
    )

    responses = []
    async for response, task in client.send_message(message):
        responses.append((response, task))

    print(json.dumps({
        "step": "send_message",
        "response_count": len(responses),
        "has_responses": len(responses) > 0,
    }))


if __name__ == "__main__":
    asyncio.run(main())
