import init, {greet, console_log, display_fibonacci_seq} from "./pkg/hello_wasm.js"



init()
.then(() => {
    console_log( "Hello WASM")
    display_fibonacci_seq( 10 )

})
.catch(
    err => console.log('WASM ERROR: ', err)
);