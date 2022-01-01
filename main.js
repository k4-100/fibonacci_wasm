import init, {greet} from "./pkg/hello_wasm.js"


init()
.then(() => {
    greet("WebAssembly")
})
.catch(
    err => console.log('WASM ERROR: ', err)
);