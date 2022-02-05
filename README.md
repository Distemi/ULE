### ULE - Minecraft's server core written in Rust
```
This's server core fully written in Rust-Lang and using more custom code
for best perfomance and controlling.
```

If you want to [contribute - i'm exists on Patreon.](https://www.patreon.com/user?u=64366641)

What's libraries using for server's core:
- ahash ( best HashMap )
- lazy_static ( Global variables )
- serde ( Serializing and Deserializing structs)
- serde_json ( convertor for JSON <-> Structs )
- log ( Logging framework )
- fern ( Logging framework's utilities )
- mio ( Single-threaded TCP and UDP server and client )