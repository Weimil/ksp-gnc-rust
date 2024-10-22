use std::fmt::Debug;
use std::process::exit;

fn dd(telemetry: impl Debug) {
    print!("{}", format!("{:?}", telemetry)
        .replace("{ ", "{\n\t")
        .replace(" }", "\n}")
        .replace(", ", ",\n\t")
    );
    exit(1);
}
fn dump(telemetry: impl Debug) {
    print!("{}", format!("{:?}", telemetry)
        .replace("{ ", "{\n\t")
        .replace(" }", "\n}")
        .replace(", ", ",\n\t")
    );
}
