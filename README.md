## `whistled` ##

`whistled` is a generic GNU/Linux application-level firewall implemented in
userspace. Its intention is to be as unopinionated as possible (with the
exception of the whole "application" concept), with all serious decision making
delegated to clients. There are instances where we might need persistent rules
(early start-up before any clients join) but the will hopefully be configurable.

Note also that this is my first proper Rust project, so I apologise if the code
is horrific -- contributions welcome!

### License ###

`whistled` is licensed under the terms of the GPLv3 (or later).

```
whistled: generic GNU/Linux application firewall daemon
Copyright (C) 2017 Aleksa Sarai <cyphar@cyphar.com>

whistled is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

whistled is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with whistled.  If not, see <http://www.gnu.org/licenses/>.
```
