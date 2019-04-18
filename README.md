# node-wait

Provide a function to block js thread for given time.

```javascript
const wait = require('node-wait')

const start = Date.now()
wait(1000)

console.log(Date.now() - start)
// 1003
```

