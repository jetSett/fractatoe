import os
import sys

if len(sys.argv) < 3:
    print(
        "Error: usage create_and_show.py <fractal_config.json> <rendering_config.json> [outfile.png]",
        file=sys.stderr,
    )
    exit(1)

fractal_config_file = sys.argv[1]
rendering_config_file = sys.argv[2]

if len(sys.argv) == 4:
    outfile = sys.argv[3]
else:
    outfile = None

histogram_filename = fractal_config_file.split("/")[-1] + ".histogram"

histogram_file = f"/tmp/fractatoe_histogram/{histogram_filename}.histogram"

os.system(f"mkdir -p /tmp/fractatoe_histogram/")

print("Generating histogram")
if os.system(
    f"cargo run --quiet --bin fractatoe_histogram_generator {fractal_config_file} {histogram_file}"
):
    exit(1)

print("Rendering image")

if outfile is not None:
    os.system(
        f"cargo run --quiet --bin fractatoe_histogram_renderer {histogram_file} {rendering_config_file} -o {outfile}"
    )
else:
    os.system(
        f"cargo run --quiet --bin fractatoe_histogram_renderer {histogram_file} {rendering_config_file}"
    )
