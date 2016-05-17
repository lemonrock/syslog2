[](This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.)
[](Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.)

# syslog2

[![Clippy Linting Result](https://clippy.bashy.io/github/lemonrock/syslog2/master/badge.svg?style=plastic)](https://clippy.bashy.io/github/lemonrock/syslog2/master/log) [![](https://img.shields.io/badge/Code%20Style-rustfmt-brightgreen.svg?style=plastic)](https://github.com/rust-lang-nursery/rustfmt#configuring-rustfmt)

[syslog2] is a rust crate that provides a correct, RFC-compliant syslog implementation and local libc wrapper; it re-uses libc functionality. It correctly models structured data (unlike other rust implementations) and support logging to standard error on Solaris and Windows (which lack such functionality natively).

A future direction is to add support for logging to Windows Event Log.

This crate supports logging to [loggly](https://www.loggly.com/docs/streaming-syslog-without-using-files/) using custom structured data.


## Licensing

The license for this project is MIT.

[syslog2]: https://github.com/lemonrock/syslog2 "syslog2 GitHub page"
