# What is this
This repo tries to give an overview over the landscape of the the different dbus implementations that exists in the rust ecosystem.
The parameter I care about is executable size when using the library.

1. https://github.com/KillingSpark/rustbus
1. https://github.com/diwic/dbus-rs/ (bindings to C library)
1. https://github.com/diwic/dbus-rs/tree/master/dbus-native
1. https://gitlab.freedesktop.org/zeenix/zbus/
1. https://github.com/Arnavion/dbus-pure
1. https://github.com/srwalter/dbus-bytestream
1. https://github.com/LinkTed/dbus-message-parser
1. https://github.com/cmaves/async-rustbus

Adapted from timing benchmarks:
https://github.com/KillingSpark/rust-dbus-comparisons

## The benchmarks
The dbus-message-parser does not provide any means of sending messages, so the result for it is pretty interesting.

## Current results
I used rust 1.64.0 to run these benchmarks.


Library             | release with send | debug | release, marshal only
--------------------|-------------------|-------|-----------------------
dbus-native         |  356K             |  6,4M |  312K
rustbus-async       |  756K             |  35M  |  348K
zvariant            |  728K             |  21M  |  428K
dbus-message-parser |  484K             |  8,9M |  352K
dbus-bytestream     |  352K             |  6,5M |  336K
dbus-pure           |  312K             |  18M  |  344K
rustbus             |  484K             |  11M  |  332K
dbus-rs             |  416K             |  9,3M |  308K

Why the hell did dbus-mesage-parser's size shoot up so much when adding sending?  
dbus-native didn't, as expected, since everything is done inside C library.  
dbus-pure shrank?  

This benchmark was created to justify my prejudice against zbus and rust async
ecosystem as a whole, and that was a success, their results are undeniably bad.
Other than that, it's uncertain.
