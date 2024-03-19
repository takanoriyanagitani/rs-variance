import { readFile } from "node:fs/promises"

(() => {
    return Promise.resolve("rs_variance.wasm")
    .then(name => readFile(name))
    .then(WebAssembly.instantiate)
    .then(pair => {
        const {
            module,
            instance,
        } = pair || {}
        const {
            memory,

            ptr32f,
            ptr64f,

            resize32f,
            resize64f,

            var32f_simple,
            var32f_simple_unbiased,

            var32f_2pass_unbiased,
            var32f_2pass_unbiased_partial64f,
            var64f_2pass_unbiased,

            var32f_shift_unbiased,
            var32f_shift_unbiased_simd128,
        } = instance?.exports || {}

        const sz = 65536

        console.info(resize32f(sz))
        console.info(resize64f(sz))
        const p32f = ptr32f()
        const p64f = ptr64f()
        const view32f = new Float32Array(memory?.buffer, p32f, sz)
        const view64f = new Float64Array(memory?.buffer, p64f, sz)

        const offset = 16777200.0

        view32f[0] =  4.0 + offset
        view32f[1] =  7.0 + offset
        view32f[2] = 13.0 + offset
        view32f[3] = 16.0 + offset

        view64f[0] =  4.0 + offset
        view64f[1] =  7.0 + offset
        view64f[2] = 13.0 + offset
        view64f[3] = 16.0 + offset

        for(let i=4; i<sz; i++){
            const m = i >> 2;
            const f5 = view32f[m];
            const f6 = view64f[m];

            view32f[i] = f5;
            view64f[i] = f6;
        }

        const results = {
            simple: var32f_simple_unbiased(),
            pass2: var32f_2pass_unbiased(),
            pass2partial64f: var32f_2pass_unbiased_partial64f(),
            shift: var32f_shift_unbiased(view32f[0]),
            shift128simd: var32f_shift_unbiased_simd128(view32f[0]),
            pass2f64: var64f_2pass_unbiased(),
        }

        console.info(results)

        const lpcnt = 16384
        const funcs = [
            {f: var32f_simple_unbiased, name: "simple"},
            {f: var32f_2pass_unbiased, name: "2pass"},
            {f: var64f_2pass_unbiased, name: "2pass64f"},
            {f: var32f_2pass_unbiased_partial64f, name: "2pass, partial 64"},
            {f: _ => var32f_shift_unbiased(view32f[0]), name: "shift"},
            {f: _ => var32f_shift_unbiased_simd128(view32f[0]), name: "shift128simd"},
        ]

        const bench = {}
        funcs.forEach(fnc => {
            const {
                f,
                name,
            } = fnc
            const started = Date.now()
            for(let i=0; i<lpcnt; i++){
                f()
            }
            const elapsed = Date.now() - started
            bench[name] = elapsed
        })

        return {bench}
    })
    .then(console.info)
    .catch(console.warn)
})()
