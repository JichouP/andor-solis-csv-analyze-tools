# andor-solis-csv-analyze-tools

Tools for analyzing csv file exported from andor solis.

## Download

[Releases · JichouP/andor-solis-csv-analyze-tools](https://github.com/JichouP/andor-solis-csv-analyze-tools/releases)

## asc-bundler

Bundle Andor Solis acquisition data (`.asc`) files into a single `.csv` file.

### Usage

#### Convert `.sif` to `.asc`

1. Launch Andor Solis and select `File > Batch Conversion` from the menu.
2. Choose acquisition data files (`.sif`) and select convert to `ASCII` format.
3. Click `Convert`. Then you will be prompted Ascii Separator window.
4. Select `Comma (,)` as Ascii Separator, enable the `Append Acquisition Information` checkbox and the `Append Information at bottom` radio button. Then click `OK`.

#### Bundle `.asc` files with `.csv`

1. Download the latest release of `asc-bundler` from [github releases](https://github.com/JichouP/andor-solis-csv-analyze-tools/releases).
2. Move `asc-bundler` executable to working directory.
3. Run `asc-bundler` executable. This will make `input` and `output` subdirectories.
4. Put the `.asc` files in `input`.
5. Run `asc-bundler`.
6. You can get the bundled `.csv` files in `output`.
