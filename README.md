# Point Cloud Solar Radiation Tool (pcsrt)
A tool for modeling solar radiation & insolation on point cloud data built in Rust.

## Description
pcsrt addresses the issue of solar radiation modeling in 3D space using generic voxel representation of space making it usable on complex objects such as vegetation. It implements the [European Solar Radiation Atlas (ESRA)](https://www.sciencedirect.com/science/article/pii/S0038092X99000559) model that uses Linke turbidity factor to estimate the attenuation of the irradiance in atmosphere. The direct (beam) and diffuse component of the solar radiation, as well as insolation count, is calculated for every point. pcsrt transforms the input point cloud into 3D voxel grid, constructs the regression planes for each voxel based on surrounding points and calculates the insolation and solar radiation components for the time period specified. The position of the Sun is calculated based on reference point (ideally centroid) of the point cloud. LAS/LAZ file formats are currently supported as input files and LAS/LAZ/PLY as output files.

## Build
1. Download the source code.
2. Install the [Rust compiler](https://www.rust-lang.org/tools/install).
3. In the directory of the pcsrt source run:
    ```
    cargo build --release
    ```

4. Compiled executable will be in `pcsrt/target/release/pcsrt`

## Usage

pcsrt is a command line tool that requires at least the point cloud centroid position, Linke turbidity factor and time period to be specified in addition to input and output file paths. However, additional optional parameters can be used to modify the way in which pcsrt process the point cloud. The most "sensitive" params are `linke_turbidity_factor` which has direct impact on output solar radiation values and `voxel_size` that specifies the detail in which the cloud is processed.

Currently LAS/LAZ file readers are implemented for input files and LAS/LAZ & PLY (binary and text) writers are implemented for output files.

---
**Note**

The output values in case of LAS/LAZ are written as `Extra Bytes Record` vlr used by [CloudCompare](https://www.danielgm.net/cc/). 

CloudCompare is also suggested for display and further processing of the point cloud.

---

```
pcsrt --input_file <path> --centroid_lat <decimal> --centroid_lon <decimal> --centroid_elev <decimal> --start_time <RFC 3339 datetime> --end_time <RFC 3339 datetime> --step_mins <integer> --linke_turbidity_factor <decimal> --output_file <path>
```

| Param                     | Type         | Required | Description                                                                        | 
| ------------------------- | ------------ | -------- | ---------------------------------------------------------------------------------- |
| --input_file              | path string  | yes      | pcsrt currently support only las/laz files as input point clouds                   | 
| --output_file             | path string  | yes      | Output file path with file extension specifying the output format (las/laz/ply)    |
| --output_ply_ascii        | flag         | no       | Specifies if the output ply file is text and not binary                            |
| --centroid_lat            | decimal      | yes      | Latitude of the centroid used as reference point in solar position calculation     |
| --centroid_lon            | decimal      | yes      | Longitude of the centroid used as reference point in solar position calculation    |
| --centroid_elev           | decimal      | yes      | Ellipsoidal elevation of the centroid                                              |
| --start_time              | RFC3339 date | yes      | Start time of the modeled period in RFC3339 format (`2020-01-01T12:00:00.000Z`) |
| --end_time                | RFC3339 date | yes      | End time of the modeled period in RFC3339 format (`2020-02-01T12:00:00.000Z`)   |
| --step_mins               | unsigned int | yes      | Step in minutes in which the solar radiation/insolation is modeled                 |
| --linke_turbidity_factor  | decimal      | yes      | Linke turbidity factor used in [ESRA  solar radiation model](https://www.sciencedirect.com/science/article/pii/S0038092X99000559) |
| --voxel_size              | decimal      | no       | Size of the voxel in meters                                                       |
| --average_points_in_voxel | decimal      | no       | If the voxel size is not specified, pcsrt will calculate it's value based on average points in voxel. If both voxel size and average points are not specified, voxel size will be calculated with 4 points in average.          |
| --block_size              | unsigned int | no       | If specified (meters), pcsrt will divide the cloud in square blocks and process them sequentially. This parameter is useful if the whole cloud does not fit in the memory.                                                       |
| --block_overlap           | unsigned int | no       | Distance in meters defining the area around block included in the calculation      |

## License
