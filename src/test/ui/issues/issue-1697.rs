// Testing that we don't fail abnormally after hitting the errors

use unresolved::*; //~ ERROR unresolved import `unresolved` [E0432]
                   //~^ maybe a missing `extern crate unresolved;`?

fn main() {}
