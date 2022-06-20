/// Memory Mapped Append-only Vector Error
///
/// ## Examples
/// ```no_run
/// let check_size = |size: usize| -> Result<(), rapiddb::errors::MMAVError> {
///     if size > 4000000 {
///         return Err(rapiddb::errors::MMAVError::FileFull);
///     }
///     if size > 9999 {
///         return Err(rapiddb::errors::MMAVError::ArrayFull);
///     }
///
///     Ok(())
/// };
///
/// check_size(10000).unwrap_or_else(|error| match error {
///     rapiddb::errors::MMAVError::FileFull => {
///         println!("handle FileFull here");
///     }
///     rapiddb::errors::MMAVError::ArrayFull => {
///         println!("handle ArrayFull here");
///     }
///     _ => (),
/// });
/// ```
pub enum MMAVError {
    FileFull,
    ArrayFull,
    ArrayEmpty,
    IndexOutOfRange,
    IndexOutOfBounds,
}
