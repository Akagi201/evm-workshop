import asyncio
import json
import time

import aiohttp
import websockets
from web3 import Web3

http_url = "https://mainnet.gateway.tenderly.co/xxx"
ws_url = "wss://ethereum-rpc.publicnode.com"

provider = Web3(Web3.HTTPProvider(http_url))
ws_provider = Web3(Web3.WebsocketProvider(ws_url))


async def geth_style_tracing(tx_hash: str):
    async with aiohttp.ClientSession() as session:
        req = {
            "id": 1,
            "method": "debug_traceTransaction",
            "jsonrpc": "2.0",
            "params": [tx_hash, {"tracer": "prestateTracer"}],
        }
        headers = {"Content-Type": "application/json"}
        request = await session.post(http_url, data=json.dumps(req), headers=headers)
        res = await request.json()
        print(res)

        result = res.get("result")

        if result:
            addresses_touched = list(result.keys())
            print("Geth style: ", addresses_touched)


async def parity_style_tracing(tx_hash: str):
    async with aiohttp.ClientSession() as session:
        req = {
            "id": 1,
            "method": "trace_replayTransaction",
            "jsonrpc": "2.0",
            "params": [tx_hash, ["stateDiff"]],
        }
        request = await session.post(http_url, data=json.dumps(req))
        res = await request.json()
        # print(res)

        result = res.get("result")

        if result:
            state_diff = result["stateDiff"]
            addresses_touched = list(state_diff.keys())
            print("Parity style: ", addresses_touched)


async def main():
    while True:
        try:
            async with websockets.connect(ws_url) as ws:
                subscription = {
                    "json": "2.0",
                    "id": 1,
                    "method": "eth_subscribe",
                    "params": ["newPendingTransactions"],
                }

                await ws.send(json.dumps(subscription))
                res = await ws.recv()
                print(res)

                while True:
                    msg = await asyncio.wait_for(ws.recv(), timeout=60 * 10)
                    response = json.loads(msg)
                    tx_hash = response["params"]["result"]

                    await geth_style_tracing(tx_hash)
                    await parity_style_tracing(tx_hash)

                    print("\n")
        except:
            time.sleep(2)
            print("reconnecting...")


if __name__ == "__main__":
    asyncio.run(main())
    # asyncio.run(
    #     geth_style_tracing(
    #         "0x015e4a799c92532803e381a1ad715af3124b297c5958bccc563bc633fe2b82ea"
    #     )
    # )
