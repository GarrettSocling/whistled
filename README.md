## `whistled` ##

`whistled` is a generic GNU/Linux application-level firewall implemented in
userspace. Its intention is to be as unopinionated as possible (with the
exception of the whole "application" concept), with all serious decision making
delegated to clients. There are instances where we might need persistent rules
(early start-up before any clients join) but the will hopefully be configurable.

Note also that this is my first proper Rust project, so I apologise if the code
is horrific -- contributions welcome!
