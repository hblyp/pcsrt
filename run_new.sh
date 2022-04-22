#!/bin/bash

cargo build --release &&
./target/release/pcsrt \
-c 49.1748,19.8756,1400.0 \
-t 2022-07-21T00:00:00.000Z,2022-07-22T00:00:00.000Z \
-s 60 \
-v 2 \
-l 2.2,2.5,3,3.1,2.9,2.9,3,2.8,2.6,2.5,2.4,2.2 \
/home/filip/phd/solar/horizon/dmr_points_sample.laz \
/home/filip/phd/solar/horizon/dmr_points_sample_172_solar_with_horizon.laz
# -h 30,37.194355,37.444151,33.501702,25.873065,15.213712,7.452704,21.489304,27.284602,17.603075,-9.927281,3.668643,24.971680 \
# --average_points_in_voxel 3 \
# --linke_turbidity_factor 3 \
# --input_file ../data/in/sucha_dolina.las \
# --output_file ../data/out/sucha_dolina_refactor.laz
# --block_size 500 \
# --block_overlap 50 \
# --output_ply_ascii false \

# todo -> vycistit, no plane for points, monthly linke