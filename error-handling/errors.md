### Errors handling
- Recoverable and unrecoverable errors
- Result<T, E> fore recoverable errors
- Panic for unrecoverable errors
- .unwrap() without message
- .expect() with message
- Unwinding the Stack or Aborting in Response to a Panic

````rust
[profile.release]
panic = 'abort'
````
- ? is only used with return type like Result and option 