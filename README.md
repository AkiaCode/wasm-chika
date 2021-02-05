# wasm-chika
WebAssembly for easy canvas manipulation

```javascript
import * as chika from "chika";

chika.Chika.clean('chika')
new chika.Text('chika', 'asd', 50, 50, 'bold 48px serif')
```

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
