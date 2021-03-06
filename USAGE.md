# K-means clustering

k-means clustering is a method of vector quantization, originally from signal processing, that aims to partition n observations into k clusters in which each observation belongs to the cluster with the nearest mean (cluster centers or cluster or cluster liner ), serving as a prototype of the cluster. This results in a partitioning of the data space into Voronoi cells. k-means clustering minimizes within-cluster variances (squared Euclidean distances), but not regular Euclidean distances, which would be the more difficult Weber problem: the mean optimizes squared errors, whereas only the geometric median minimizes Euclidean distances. For instance, better Euclidean solutions can be found using k-medians and k-medoids.

The problem is computationally difficult (NP-hard); however, efficient heuristic algorithms converge quickly to a local optimum. These are usually similar to the expectation-maximization algorithm for mixtures of Gaussian distributions via an iterative refinement approach employed by both k-means and Gaussian mixture modeling. They both use cluster centers to model the data; however, k-means clustering tends to find clusters of comparable spatial extent, while the Gaussian mixture model allows clusters to have different shapes.

The unsupervised k-means algorithm has a loose relationship to the k-nearest neighbor classifier, a popular supervised machine learning technique for classification that is often confused with k-means due to the name. Applying the 1-nearest neighbor classifier to the cluster centers obtained by k-means classifies new data into the existing clusters. This is known as nearest centroid classifier or Rocchio algorithm. 

# Input

|param|type|description|required|
|--------|----|-----------|--------|
|`data`|list[list[number]]|A n x n matrix represented as a list of lists where each row is a vector to be categorized into a cluster|yes|
|`k`|number|A number representing the amount of clusters to identify|yes|

# Output

|type|description|
|----|-----------|
|list[number]|A list representing the cluster associated to each `data` row|

# Driver example (python)

```python
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
# [0, 0, 0, 0, 0, 1, 1, 1, 1, 1]
```
