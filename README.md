# Rust Backend Practice

## 跨域相关问题
* `Access-Control-Allow-Origin: *` 表示服务器允许任何源的网页进行跨源请求。
* `Access-Control-Allow-Credentials: true` 表示请求应该包含凭据，比如cookies。

当你尝试同时设置`Access-Control-Allow-Origin: *`和`Access-Control-Allow-Credentials: true`时，问题就出现了。 解决方法：
* 如果你想要允许所有源但不携带凭据，你可以设置`Access-Control-Allow-Origin: *`和`Access-Control-Allow-Credentials: false`。
* 如果你想要允许特定的源并携带凭据，你可以设置`Access-Control-Allow-Origin`为特定的源，并且`Access-Control-Allow-Credentials: true`。


## 状态码
### 401和403的区别
* `401 Unauthorized`：表示认证失败，请求没有被认证或者认证失败。
  * token失效、token缺失、token伪造，导致服务端无法识别身份。
* `403 Forbidden`：表示授权失败，通常表示用户通过了身份验证，但缺少权限对给定的资源进行访问或者操作。
  * 用户认证成功，但是无权进行操作。



