# zeroize-expose
Demonstrates the potential to expose a secret in memory before initial object has been zeroized.

## Issue
[Zeroizing](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) an object in memory is an effective and secure way to purge secrets from the heap or stack, but one has to be careful not to inadvertently expose the secret before calling [zeroize](https://crates.io/crates/zeroize) on the object.

Exposed secrets can be tricky to catch during a code review if a copy of the object is made before it is zeroed because there may be assumptions that zeroing the object handles every instance of the secret in memory prior to calling zeroize.

For example, one way to expose a secret stored in the heap could be as harmless as the use of a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html) like `println!` or `print!` before calling `zeroize()` on the object. 

Note the documentation for the `zeroize` crate states the following disclaimer under [Stack/Heap Zeroing Notes](https://docs.rs/zeroize/1.7.0/zeroize/#stackheap-zeroing-notes)

*"Be aware several operations in Rust can unintentionally leave copies of data in memory. This includes but is not limited to
the following:"*

 - Moves and `Copy`
 - Heap reallocation when using `Vec` and `String`
 - Borrowers of a reference making copies of data

The `zeroize` crate only guarantees that *"subsequent reads result in zeroized values"*, but it has no way of zeroing secrets that are either deliberately or inadvertently exposed. 

The potential for exposure is not limited to `String` type, but can also apply to a `Vec`.

## Intent
Create awareness around securely managing secrets and the potential for exposure even when relying on the use of `zeroize`

This project was inspired by the `memory-testing` crate in the bitwarden [sdk](https://github.com/bitwarden/sdk/) and demonstrates the ease at which one can undermine zeroing.

## Usage
./zeroize-test expose

Generates a secret and exposes the data in pre and post zeroized memory dumps.

./zeroize-test zero

Generates a secret and performs a secure zeroing operation on the object.

