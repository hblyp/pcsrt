# Point Cloud Solar Radiation Tool (pcsrt)
A tool for modeling solar radiation & insolation on point cloud data built in Rust.

![](/img/preview.png)

## Description
pcsrt addresses the issue of solar radiation modeling in 3D space using generic voxel representation of space making it usable on complex objects such as vegetation. It implements the [European Solar Radiation Atlas (ESRA)](https://www.sciencedirect.com/science/article/pii/S0038092X99000559) model that uses Linke turbidity factor to estimate the attenuation of the irradiance in atmosphere. The direct (beam) and diffuse component of the solar radiation, as well as insolation count, is calculated for every point. pcsrt transforms the input point cloud into 3D voxel grid, constructs the regression planes for each voxel based on surrounding points and then calculates the insolation and solar radiation components for the time period specified. The position of the Sun is calculated based on reference point (ideally centroid) of the point cloud. LAS/LAZ file formats are currently supported as input files and LAS/LAZ/PLY as output files.

## Build
1. Download the source code.
2. Install the [Rust compiler](https://www.rust-lang.org/tools/install).
3. In the directory of the pcsrt source run:
    ```
    cargo build --release
    ```

4. Compiled executable will be in `pcsrt/target/release/pcsrt`

**Note:** Building from source on Windows requires [C++ Build Tools](https://docs.microsoft.com/en-us/windows/dev-environment/rust/setup). 

## Usage

pcsrt is a command line tool that requires at least the point cloud centroid position, Linke turbidity factor and time period to be specified in addition to input and output file paths. However, additional optional parameters can be used to modify the way in which pcsrt processes the point cloud. The most "sensitive" params are `--linke-turbidity-factor` which has direct impact on output solar radiation values and `--voxel-size` that specifies the detail in which the cloud is processed.

Output point cloud contains irradiation values [W.h/m^2] - `global_irradiance`, `beam_component`, `diffuse_component` and the `illumination_count` in the time period with the time step. 

Currently LAS/LAZ file readers are implemented for input files and LAS/LAZ & PLY (binary and text) writers are implemented for output files.

**Note:** The output values in case of LAS/LAZ are written as `Extra Bytes Record` vlr used by [CloudCompare](https://www.danielgm.net/cc/). CloudCompare is also suggested for display and further editing of the point cloud.

```
pcsrt [OPTIONS] --centroid <CENTROID> --time-range <TIME_RANGE> --step-mins <STEP_MINS> --linke-turbidity-factor <LINKE_TURBIDITY_FACTOR> <INPUT_FILE> <OUTPUT_FILE>
```

| Param                         | Type/Format                                                         | Required | Description                                                                        | 
| ----------------------------- | ------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------------- |
| -c, --centroid                | <LAT(float)>,<LON(float)>,<ELEVATION(float)>                        | yes      | Point cloud centroid geographical coordinates & ellipsoidal elevation |
| -t, --time-range              | <FROM(2020-01-01T12:00:00.000Z)>,<TO(2020-03-23T18:00:00.000Z)>     | yes      | Time range in RFC3339 format |
| -s, --step-mins               | int                                                                 | yes      | Step in minutes used in time range |
| -l, --linke-turbidity-factor  | <SINGLE_LINKE(float)> or <MONTHLY_LINKE(12 comma separated floats)> | yes      | Linke turbidity factor used in [ESRA  solar radiation model](https://www.sciencedirect.com/science/article/pii/S0038092X99000559) (single value or 12 monthly values) |
| -h, --horizon                 | <ANGLE_STEP(int)>,<ELEVATION(float,float,...)>                      | no       | Horizon height used to take in account surrounding horizon (hills) when modeling solar radiation in smaller areas. Starts from north. (GRASS [r.horizon](https://grass.osgeo.org/grass80/manuals/r.horizon.html) is a useful tool for this parameter, you can also find it as r.horizon.height in QGIS) [default: 360,0] |
| -v, --voxel-size              | decimal                                                             | no       | Size of the voxel in meters |
| -p, --average-points-in-voxel | decimal                                                             | no       | Instead of specifing voxel size, average points in voxel can be used. [default: 4] |
| -b, --block-process-params    | <SIZE(int)>,<OVERLAP(int)>                                          | no       | If specified (meters), pcsrt will divide the cloud in square blocks and process them sequentially. This parameter is useful if the whole cloud does not fit in the memory. |
| --output_ply_ascii            | flag                                                                | no       | When using ply output, specify if using binary (default) or text format |

## License
PCSRT is [MIT licensed](/LICENSE).