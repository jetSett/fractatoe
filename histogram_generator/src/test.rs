use super::*;

#[test]
fn test_julia_conf() {
    read_config_file("../samples/julia.json").unwrap();
}

#[test]
fn test_mandelbrot_conf() {
    read_config_file("../samples/mandelbrot.json").unwrap();
}

#[test]
fn test_sierpinsky_conf() {
    read_config_file("../samples/sierpinsky.json").unwrap();
}
