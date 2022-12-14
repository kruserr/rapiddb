use crate::errors::MMAVError;

/// Memory Mapped Append-only Vector Unit
///
/// This uses memory mapped file io, to write to disk, as if it were
/// regular memory.
/// It implements a simple statically sized append-only vector.
///
/// ## Examples
/// ```ignore
/// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
///
/// let data = b"{\"key\":\"value\"}";
/// unit.push(data).unwrap_or_default();
/// assert_eq!(unit.last(), data);
/// ```
pub struct MMAVUnit {
  seek: usize,
  seek_index: usize,
  mmap: memmap2::MmapMut,
  data_start_index: usize,
}
impl MMAVUnit {
  /// Memory Mapped Append-only Vector Unit Constructor
  ///
  /// ## Default params:
  ///
  /// `size` = 4000000
  ///
  /// `data_start_index` = 80008
  ///
  /// ## Examples
  /// ```ignore
  /// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// unit.push(data).unwrap_or_default();
  /// assert_eq!(unit.last(), data);
  /// ```
  pub fn new(file_name: &str, size: usize, data_start_index: usize) -> Self {
    let file_did_exist = std::path::Path::new(file_name).exists();

    let file = std::fs::OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(file_name)
      .unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
          std::fs::create_dir(file_name.split('/').collect::<Vec<_>>()[0])
            .unwrap_or_default();
        }

        std::fs::OpenOptions::new()
          .read(true)
          .write(true)
          .create(true)
          .open(file_name)
          .unwrap()
      });

    file.set_len(size as u64).unwrap_or_default();

    let mut mmap = unsafe { memmap2::MmapMut::map_mut(&file).unwrap() };
    mmap.advise(memmap2::Advice::Random).unwrap_or_default();

    let mut seek = data_start_index;
    if file_did_exist {
      seek = u32::from_ne_bytes(mmap[0..4].try_into().unwrap()) as usize;

      if seek > mmap.len() {
        panic!(
          "seek_index must be between {data_start_index} and {}",
          mmap.len()
        );
      }
    }

    let mut seek_index = 8;
    if file_did_exist {
      seek_index = u32::from_ne_bytes(mmap[4..8].try_into().unwrap()) as usize;

      if seek_index > data_start_index {
        panic!("seek_index must be between 8 and {data_start_index}");
      }
    }

    if mmap[seek] == 0 {
      mmap[seek] = 0;
    }

    Self { seek, seek_index, mmap, data_start_index }
  }

  /// Set seek to `len`
  ///
  /// ## Examples
  /// ```ignore
  /// let value: [u8; 0] = Default::default();
  /// self.set_seek(value.len());
  /// ```
  fn set_seek(&mut self, len: usize) {
    let end = self.seek + len;

    self.mmap[self.seek_index..(self.seek_index + 4)]
      .clone_from_slice(&(self.seek as u32).to_ne_bytes());
    self.mmap[(self.seek_index + 4)..(self.seek_index + 8)]
      .clone_from_slice(&(end as u32).to_ne_bytes());

    self.seek_index += 8;
    self.mmap[4..8].clone_from_slice(&(self.seek_index as u32).to_ne_bytes());

    self.seek = end;
    self.mmap[0..4].clone_from_slice(&(self.seek as u32).to_ne_bytes());
  }

  /// Push `value` to vector
  ///
  /// ## Errors
  /// ```ignore
  /// MMAVError::ArrayFull
  /// MMAVError::FileFull
  /// ```
  ///
  /// ## Examples
  /// ```ignore
  /// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// unit.push(data).unwrap_or_default();
  /// assert_eq!(unit.last(), data);
  /// ```
  pub fn push(&mut self, value: &[u8]) -> Result<(), MMAVError> {
    if self.len() > 9999 {
      return Err(MMAVError::ArrayFull);
    }

    if self.seek + value.len() > self.mmap.len() {
      return Err(MMAVError::FileFull);
    }

    self.mmap[self.seek..self.seek + value.len()].clone_from_slice(value);
    self.set_seek(value.len());

    Ok(())
  }

  /// Get `index` from vector
  ///
  /// ## Errors
  /// ```ignore
  /// MMAVError::ArrayEmpty
  /// MMAVError::IndexOutOfRange
  /// MMAVError::IndexOutOfBounds
  /// ```
  ///
  /// ## Examples
  /// ```ignore
  /// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// unit.push(data).unwrap_or_default();
  /// assert_eq!(unit.get(0), data);
  /// ```
  pub fn get(&self, index: usize) -> Result<Vec<u8>, MMAVError> {
    if self.seek_index == 8 {
      return Err(MMAVError::ArrayEmpty);
    }

    if index > 9999 {
      return Err(MMAVError::IndexOutOfRange);
    }

    if index > self.len() - 1 {
      return Err(MMAVError::IndexOutOfBounds);
    }

    let i = 8 * index + 8;

    let start =
      u32::from_ne_bytes(self.mmap[i..i + 4].try_into().unwrap()) as usize;
    let end =
      u32::from_ne_bytes(self.mmap[i + 4..i + 8].try_into().unwrap()) as usize;

    if start < self.data_start_index || start > self.mmap.len() {
      return Err(MMAVError::IndexOutOfRange);
    }

    if end < self.data_start_index || end > self.mmap.len() {
      return Err(MMAVError::IndexOutOfRange);
    }

    Ok((self.mmap[start..end]).to_vec())
  }

  /// Get last item in vector
  ///
  /// This function gracefully fails by returning an empty byte slice
  ///
  /// ## Examples
  /// ```ignore
  /// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// unit.push(data).unwrap_or_default();
  /// assert_eq!(unit.last(), data);
  /// ```
  pub fn last(&self) -> Vec<u8> {
    if self.len() == 0 {
      return Default::default();
    }

    self.get(self.len() - 1).unwrap_or_default()
  }

  /// Get the length of the vector
  ///
  /// ## Examples
  /// ```ignore
  /// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// unit.push(data).unwrap_or_default();
  /// assert_eq!(unit.len(), 1);
  /// ```
  pub fn len(&self) -> usize {
    if self.seek_index == 8 || self.seek_index == 0 {
      return 0;
    }

    (self.seek_index - 8) / 8
  }
}
