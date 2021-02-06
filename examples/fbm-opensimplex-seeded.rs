//! An example of using the fBm noise function

extern crate noise;

use noise::{utils::*, Fbm_OpenSimplex, Seedable};

fn main() {
    let fbm = Fbm_OpenSimplex::new();

    PlaneMapBuilder::new(&fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm.png");
}
