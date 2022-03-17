import asyncio
from motivus.client import Client

data = [
    [-2.7825343, -1.7604825, -5.5550113, -2.9752946, -2.7874138],
    [-2.9847919, -3.8209332, -2.1531757, -2.2710119, -2.3582877],
    [-3.0109320, -2.2366132, -2.8048492, -1.2632331, -4.5755581],
    [-2.8432186, -1.0383805, -2.2022826, -2.7435962, -2.0013399],
    [-2.6638082, -3.5520086, -1.3684702, -2.1562444, -1.3186447],
    [1.7409171, 1.9687576, 4.7162628, 4.5743537, 3.7905611],
    [3.2932369, 2.8508700, 2.5580937, 2.0437325, 4.2192562],
    [2.5843321, 2.8329818, 2.1329531, 3.2562319, 2.4878733],
    [2.1859638, 3.2880048, 3.7018615, 2.3641232, 1.6281994],
    [2.6201773, 0.9006588, 2.6774097, 1.8188620, 1.6076493],
]

task_definition = {
    'algorithm': "kmeans",
    'algorithm_version': "0.0.1",
    # 'wasm_path': "build/kmeans-0.0.1.wasm",
    # 'loader_path': "build/kmeans-0.0.1.js",
    'params': [
        data,
        2
    ]
}

async def main():
    motivus = await Client.connect()

    task_id = motivus.call_async(task_definition)
    task = motivus.select_task(task_id)
    return await task

result = asyncio.run(main())
print(result)
