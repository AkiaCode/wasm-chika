# wasm-chika
WebAssembly for easy canvas manipulation

## Example

Use https://www.npmjs.com/package/create-wasm-app  

`npm i wasm-chika` and `npm run start` is starting work

### Insert:

`index.js`
```javascript
import * as wasm from "wasm-chika";

wasm.Chika.clean('chika')
new wasm.Text('chika', 'asd', 50, 50, 'bold 48px serif')
```

`index.html`
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Hello Chika!</title>
  </head>
  <body>
    <noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
    <script src="./bootstrap.js"></script>
    <canvas id="chika"></canvas>
  </body>
</html>
```

