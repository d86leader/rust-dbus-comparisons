# What is this
This repo tries to give an overview over the landscape of the the different dbus implementations that exists in the rust ecosystem.

1. https://github.com/KillingSpark/rustbus
1. https://github.com/diwic/dbus-rs
1. https://gitlab.freedesktop.org/zeenix/zbus/
1. https://github.com/Arnavion/dbus-pure
1. https://github.com/srwalter/dbus-bytestream
1. https://github.com/LinkTed/dbus-message-parser

Note that I am the author of rustbus, but of course I am trying to be as objectiv as possible here.

## Current state
Some benchmarks exist. I plan to add equivalent ones for the missing libs, and more kinds of benchmarks.

## The benchmarks
1. Marshal: Build a signal message and marshal it
1. Marshal + Send: Connect to the sessionbus, build a signal and send it to the bus

The Marshal + Send benchmark is not performed for zvariant because the zbus library currently uses a lot of println!() 
which hamper performance unfairly. The dbus-message-parser does not provide any means of sending messages.

## Current results
I am running this on an older AMD Cpu (/proc/cpuinfo says: AMD FX(tm)-6300 Six-Core Processor). Your values might vary a bit.

To replicate these results just run: `cargo bench`. That will run all benchmarks. Alternatively you can rerun the benchmarks with more samples to get 
more reliable results. I used these parameters on the AMD CPU: `target/release/deps/marshal_bench-79fa1eab77a57d63 --nresamples 1000 --sample-size 1000 --bench`.

| Library             | Marshal                                 | Marshal + Send                          |
|---------------------|-----------------------------------------|-----------------------------------------|
| rustbus             | time:   [49.464 us 49.547 us 49.638 us] | time:   [477.34 us 478.94 us 480.64 us] |
| dbus-bytestream     | time:   [58.500 us 58.573 us 58.659 us] | time:   [401.24 us 403.00 us 404.86 us] |
| zvariant            | time:   [77.499 us 77.575 us 77.648 us  |                                         |
| dbus-pure           | time:   [95.952 us 96.030 us 96.105 us] | time:   [563.08 us 564.72 us 566.35 us] |
| dbus-message-parser | time:   [159.77 us 159.89 us 160.01 us] |                                         |
| dbus-rs             | time:   [452.14 us 452.30 us 452.45 us] | time:   [923.50 us 926.74 us 930.26 us] |

