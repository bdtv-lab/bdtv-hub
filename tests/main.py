import asyncio

from websockets.asyncio.client import connect

URI = "ws://127.0.0.1:9001"


async def main():
    async with connect(URI) as ws:
        for text in ["hello", "world"]:
            await ws.send(text)
            print(f"{text} -> {await ws.recv()}")


if __name__ == "__main__":
    asyncio.run(main())
