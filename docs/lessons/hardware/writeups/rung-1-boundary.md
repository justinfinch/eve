# Rung 1 write-up — the host↔device boundary and the bridge

**What I built.** The Pico announces `{id, name, capabilities:[{kind:"gpio",...}]}` over USB
serial. A Rust **bridge** owns the serial port and relays to the browser over a WebSocket. The
browser renders an LED control *from the announced capability* and sends back an addressed command
`(capability, channel, op, args)`; the Pico validates it and drives GP15.

**Where the boundary sits.** The browser never touches hardware. It speaks one protocol
(WebSocket) to an API. Behind that API is either the simulator (fabricated) or the bridge (a real
Pico). This is the Adom pattern — physical resources behind a software API — and it means the sim
and the real board are interchangeable peers. For a single-user desk setup the bridge *is* the
backend-for-frontend; a separate server tier only earns its place once something must live
server-side (an LLM API key, auth, remote access).

**Framing — and why it only matters on the serial link.** A WebSocket hands you whole messages; a
serial port hands you a raw byte stream with no message boundaries. So on the serial link I delimit
each message with a newline (`\n`); on the WebSocket I don't need to. The bridge's whole job is
translating between the two. The same "buffer bytes, drain complete lines, keep the partial tail"
logic shows up in three places that touch a raw stream: the bridge relay, and the firmware's read
loop.

**Plug-and-play via the capability list.** The board describes what it can do; the workbench draws
controls from that description. Nothing in the browser hardcodes "the LED button" — it draws a
control for whatever GPIO the board advertises, and shows "no gpio capability advertised" if the
list is empty. At Rung 3 an ADC readout should appear the same way, from its announcement.

**The shared-validator seam.** `resolve_gpio_set` lives once in the `contract` crate and is called
by *both* the simulator and the firmware. Fake board and real board enforce identical invariants
(address only advertised capabilities, channel in range, op advertised) because it is literally the
same code. Only the success branch differs: the sim flips a boolean; the firmware drives 3.3V on a
physical pin.

**Result on the bench.** <fill in: did the real LED respond to the browser toggle? did the
capability round-trip into the UI? what surprised me — enumeration, timing, a first-flash quirk?>
