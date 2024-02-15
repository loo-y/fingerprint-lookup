## How to use

### build
```bash
# debug
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/fingerprint_lookup.wasm --out-dir ./output
```


```bash
# release
wasm-pack build --target web
wasm-bindgen target/wasm32-unknown-unknown/release/fingerprint_lookup.wasm --out-dir ./output
```
                                                              
### import in React
```javascript
import { __wbg_set_wasm, check_environment, get_browser_info } from "./fingerprint_lookup_bg";
import * as fingerprint_lookup_bg_js from './fingerprint_lookup_bg'

...
...

  useEffect(()=>{
    const checkenv = async()=>{
      const response = await fetch("./fingerprint_lookup_bg.wasm")
      const bytes = await response.arrayBuffer();
      const { instance } = await WebAssembly.instantiate(bytes, {'./fingerprint_lookup_bg.js': fingerprint_lookup_bg_js});
      __wbg_set_wasm(instance.exports);
      // @ts-ignore
      check_environment()
      // @ts-ignore
      const ua = get_browser_info()
      console.log(`ua`, ua)
    }
    checkenv()
  }, [])

...
...
```