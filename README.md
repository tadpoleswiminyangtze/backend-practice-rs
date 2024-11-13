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


## Rust相关
### and_then
Returns None if the option is None, otherwise calls f with the wrapped value and returns the result.
```rust
pub fn and_then<U, F>(self, f: F) -> Option<U>
where
    F: FnOnce(T) -> Option<U>,
{
    match self {
        Some(x) => f(x),
        None => None,
    }
}
```
* 作用于Option
* 如果是Some，则用Some中的值作为参数调用传入的函数
* 如果是None，则什么也不做
* 专门处理有值的情况


### or_else
Returns the option if it contains a value, otherwise calls f and returns the result.
```rust
pub fn or_else<F>(self, f: F) -> Option<T>
where
    F: FnOnce() -> Option<T>,
{
    match self {
        x @ Some(_) => x,
        None => f(),
    }
}
```
* 作用于Option
* 如果是Some，则什么也不做
* 如果是None，则调用传入的函数
* 专门处理无值的情况


### map
Maps a `Result<T, E>` to `Result<U, E>` by applying a function to a contained [`Ok`] value, leaving an [`Err`] value untouched.
```rust
pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Result<U, E> {
    match self {
        Ok(t) => Ok(op(t)),
        Err(e) => Err(e),
    }
}
```
* 作用于`Result`
* 如果是`Ok`，则用`Ok`中的值作为参数调用map的函数
* 如果是`Err`则什么都不做
* 专门处理`Ok`的情况


### map_err
Maps a `Result<T, E>` to `Result<T, F>` by applying a function to a contained [`Err`] value, leaving an [`Ok`] value untouched.
```rust
pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
    match self {
        Ok(t) => Ok(t),
        Err(e) => Err(op(e)),
    }
}
```
* 作用于Result
* 如果是Ok，则什么也不做
* 如果是Err，则调用map的传入的函数
* 专门处理`Err`的情况


### map_or
Returns the provided default (if [`Err`]), or applies a function to the contained value (if [`Ok`]).
```rust
pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
    match self {
        Ok(t) => f(t),
        Err(_) => default,
    }
}
```
* 作用于`Result`
* 如果是`Ok`，则用`Ok`中的值作为参数调用map的函数
* 如果是`Err`则返回默认值


### map_or_else
Maps a `Result<T, E>` to `U` by applying fallback function `default` to a contained [`Err`] value, or function `f` to a contained [`Ok`] value.
```rust
pub fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
    match self {
        Ok(t) => f(t),
        Err(e) => default(e),
    }
}
```
* 作用于`Result`
* 如果是`Ok`，则用`Ok`中的值作为参数调用map的第二个函数
* 如果是`Err`则调用map中的第一个函数

`map_or`和`map_or_else`的区别：
* `map_or`的第一个参数是值（Err时用），第二个参数是函数（Ok时用）
* `map_or_else`的第一个参数是函数（Err时用），第二个参数是函数（Ok时用）



