// Base interface for a build manifest.
trait Manifest {

    fn to_string(&self) -> String;

    // This function detects if a file path could be a potential candidate
    // for this build system.
    fn path_is_manifest_type(&self);
}
