import asyncio
import json
import time

from websockets.asyncio.client import connect

URI = "ws://127.0.0.1:7497"
PLAYER_ID = "player-1"
INTERVAL = 5


async def main():
    async with connect(URI) as ws:
        while True:
            await ws.send(
                json.dumps(
                    {
                        "action": "heartbeat",
                        "data": {"id": PLAYER_ID, "timestamp": int(time.time())},
                    }
                )
            )
            print(f"heartbeat sent as {PLAYER_ID}")
            await asyncio.sleep(INTERVAL)


if __name__ == "__main__":
    asyncio.run(main())
