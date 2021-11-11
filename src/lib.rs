#[derive(Debug, PartialEq)]
enum BuildModel {
    Debug,
    Release,
}

fn build_channel() -> BuildModel {
    if cfg!(debug_assertions) {
        return BuildModel::Debug;
    }
    BuildModel::Release
}

pub fn is_debug() -> bool {
    build_channel() == BuildModel::Debug
}

pub fn is_release() -> bool {
    build_channel() == BuildModel::Release
}
