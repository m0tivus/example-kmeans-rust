# K-means Motivus algorithm

### 🐑 Use `wasm-pack new` to use a Motivus Algorithm template

```
wasm-pack new kmeans --template https://github.com/m0tivus/wasm-pack-template.git
cd kmeans
```

## 🚴 Development Environment Requirements
* Docker
   * Your user must belong to the `docker` group.
* Python = 3.7 | 3.8 | 3.9
   * We recommend using a `conda` environment.
* [*Motivus CLI tool* and *Motivus Client library*](https://pypi.org/project/motivus/): `$ pip install motivus`

### 🛠️ Build with `motivus build`

```
$ motivus build
```

### 🔬 Test locally with `motivus loopback`

```
$ motivus loopback
```
```
$ WEBSOCKET_URI=ws://localhost:7070/client_socket/websocket python driver.py
```

### 🎁 Publish to Motivus Marketplace with `motivus push`

```
$ motivus push
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
