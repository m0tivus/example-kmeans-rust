import asyncio
import numpy as np

from motivus.client import Client







algorithm = {
    #'algorithm': "kmeans",
    #'algorithm_version': "0.0.1",
    'wasm_path': "build/kmeans-0.0.1.wasm",
    'loader_path': "build/kmeans-0.0.1.js",
    
}

async def main():
    motivus = await Client.connect()
    task_ids = []

    for i in range(10):
        data = np.random.randint(-20, 20, (1000, 1000)).tolist()
        metadata = {
            'params': [
                data,
                150
            ]
        }
        metadata.update(algorithm)
        task_id = motivus.call_async(metadata)
        task_ids.append(task_id)

    return await motivus.barrier(task_ids)

result = asyncio.run(main())
print(result)
