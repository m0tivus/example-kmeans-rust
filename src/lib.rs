mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Macro to implement kmeans for both f64 and f32 without writing everything
// twice or importing the `num` crate
macro_rules! impl_kmeans {
    ($kind: ty, $modname: ident) => {
        // Since we can't overload methods in rust, we have to use namespacing
        pub mod $modname {
            use std::$modname::INFINITY;

            /// computes sum of squared deviation between two identically sized vectors
            /// `x`, and `y`.
            fn distance(x: &[$kind], y: &[$kind]) -> $kind {
                x.iter()
                    .zip(y.iter())
                    .fold(0.0, |dist, (&xi, &yi)| dist + (xi - yi).powi(2))
            }

            /// Returns a vector containing the indices z<sub>i</sub> in {0, ..., K-1} of
            /// the centroid nearest to each datum.
            fn nearest_centroids(xs: &[Vec<$kind>], centroids: &[Vec<$kind>]) -> Vec<usize> {
                xs.iter()
                    .map(|xi| {
                        // Find the argmin by folding using a tuple containing the argmin
                        // and the minimum distance.
                        let (argmin, _) = centroids.iter().enumerate().fold(
                            (0_usize, INFINITY),
                            |(min_ix, min_dist), (ix, ci)| {
                                let dist = distance(xi, ci);
                                if dist < min_dist {
                                    (ix, dist)
                                } else {
                                    (min_ix, min_dist)
                                }
                            },
                        );
                        argmin
                    })
                    .collect()
            }

            /// Recompute the centroids given the current clustering
            fn recompute_centroids(
                xs: &[Vec<$kind>],
                clustering: &[usize],
                k: usize,
            ) -> Vec<Vec<$kind>> {
                let ndims = xs[0].len();

                // NOTE: Kind of inefficient because we sweep all the data from each of the
                // k centroids.
                (0..k)
                    .map(|cluster_ix| {
                        let mut centroid: Vec<$kind> = vec![0.0; ndims];
                        let mut n_cluster: $kind = 0.0;
                        xs.iter().zip(clustering.iter()).for_each(|(xi, &zi)| {
                            if zi == cluster_ix {
                                n_cluster += 1.0;
                                xi.iter().enumerate().for_each(|(j, &x_ij)| {
                                    centroid[j] += x_ij;
                                });
                            }
                        });
                        centroid.iter().map(|&c_j| c_j / n_cluster).collect()
                    })
                    .collect()
            }

            /// Assign the N D-dimensional data, `xs`, to `k` clusters using K-Means clustering
            pub fn kmeans(xs: Vec<Vec<$kind>>, k: usize) -> Vec<usize> {
                assert!(xs.len() >= k);

                // Rather than pulling in a dependency to radomly select the staring
                // points for the centroids, we're going to deterministally choose them by
                // slecting evenly spaced points in `xs`
                let n_per_cluster: usize = xs.len() / k;
                let centroids: Vec<Vec<$kind>> =
                    (0..k).map(|j| xs[j * n_per_cluster].clone()).collect();

                let mut clustering = nearest_centroids(&xs, &centroids);

                loop {
                    let centroids = recompute_centroids(&xs, &clustering, k);
                    let new_clustering = nearest_centroids(&xs, &centroids);

                    // loop until the clustering doesn't change after the new centroids are computed
                    if new_clustering
                        .iter()
                        .zip(clustering.iter())
                        .all(|(&za, &zb)| za == zb)
                    {
                        // We need to use `return` to break out of the `loop`
                        return clustering;
                    } else {
                        clustering = new_clustering;
                    }
                }
            }
        }
    };
}

// generate code for kmeans for f32 data
impl_kmeans!(f32, f32);

use crate::f32::kmeans;

#[wasm_bindgen]
pub fn main(input: &JsValue, clusters: &JsValue) -> Box<[JsValue]> {
    // utils::set_panic_hook();

    let xs: Vec<Vec<f32>> = input.into_serde().unwrap();
    let k: usize = clusters.into_serde().unwrap();
    let clustering = kmeans(xs, k);

    clustering
        .into_iter()
        .map(|s| s as u32)
        .map(JsValue::from)
        .collect::<Vec<JsValue>>()
        .into_boxed_slice()
}
