/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Some crumminess to make sure we link correctly

#[cfg(target_os = "linux")]
#[link_args = "-L. -lazure -lstdc++ -lskia -lfontconfig"]
#[nolink]
extern mod m { }

#[cfg(target_os = "macos")]
#[link_args = "-L. -lazure -lstdc++ -framework ApplicationServices \
			   -lskia -framework IOSurface -lobjc -framework OpenGL \
			   -framework Foundation -framework QuartzCore"]
#[nolink]
extern mod m { }
