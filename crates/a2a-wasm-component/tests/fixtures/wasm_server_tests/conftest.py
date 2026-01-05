import os
import pytest


@pytest.fixture
def server_url():
    """Get the WASM server URL from environment."""
    url = os.environ.get("WASM_SERVER_URL", "http://localhost:9998")
    return url
