fn main() {
    compile_proto("proto/patient.proto");
    compile_proto("proto/appointment.proto");
}

fn compile_proto(path: &str) {
    tonic_build::compile_protos(path).unwrap_or_else(|e| panic!("Failed to compile proto {:?}", e))
}
