//! An example of using the fBm noise function

extern crate noise;

use noise::{utils::*, Fbm, Seedable};

fn main() {
    let mut fbm = Fbm::new();
    fbm = fbm.set_seed(1);

    PlaneMapBuilder::new(&fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm-1.png");
}
