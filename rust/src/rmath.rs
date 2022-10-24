/// Functions here are based on math functions from base R.
/// The benefits of porting are that it is:
/// - more fun than to fiddle with C includes,
/// - is an opportunity for learning,
/// - is more flexible when building to different platforms.
///
/// The reason that these functions are not often distributed is
/// that noncentral distributions are mostly useful for power analyses.

mod utils;
mod qnt;
mod pnt;
