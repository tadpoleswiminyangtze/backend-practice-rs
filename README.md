# Rust Backend Practice


* `Access-Control-Allow-Origin: *` 表示服务器允许任何源的网页进行跨源请求。
* `Access-Control-Allow-Credentials: true` 表示请求应该包含凭据，比如cookies。

当你尝试同时设置`Access-Control-Allow-Origin: *`和`Access-Control-Allow-Credentials: true`时，问题就出现了。 解决方法：
* 如果你想要允许所有源但不携带凭据，你可以设置`Access-Control-Allow-Origin: *`和`Access-Control-Allow-Credentials: false`。
* 如果你想要允许特定的源并携带凭据，你可以设置`Access-Control-Allow-Origin`为特定的源，并且`Access-Control-Allow-Credentials: true`。