import init, {greet, console_log} from "./pkg/hello_wasm.js"



init()
.then(() => {
    console_log( "Hello WASM")
})
.catch(
    err => console.log('WASM ERROR: ', err)
);