use crate::errors::Error;

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
  pub fn new(
    file_name: &str,
    size: usize,
    data_start_index: usize,
  ) -> Result<Self, Error> {
    let file_path = std::path::Path::new(file_name);
    let file_exists = file_path.exists();

    if (!file_exists) {
      file_path.parent().map(std::fs::create_dir_all);
    }

    let file = std::fs::OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .truncate(false)
      .open(file_name)?;

    file.set_len(size as u64)?;

    let mut mmap = unsafe { memmap2::MmapMut::map_mut(&file)? };
    mmap.advise(memmap2::Advice::Random).unwrap_or_default();

    let mut seek = data_start_index;
    if (file_exists) {
      seek = u32::from_ne_bytes(mmap[0..4].try_into()?) as usize;

      if seek > mmap.len() {
        return Err(Error::IndexOutOfRange);
      }
    }

    let mut seek_index = 8;
    if (file_exists) {
      seek_index = u32::from_ne_bytes(mmap[4..8].try_into()?) as usize;

      if seek_index > data_start_index {
        return Err(Error::IndexOutOfRange);
      }
    }

    if (mmap[seek] == 0) {
      mmap[seek] = 0;
    }

    return Ok(Self { seek, seek_index, mmap, data_start_index });
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
  /// ## Examples
  /// ```ignore
  /// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// unit.push(data).unwrap_or_default();
  /// assert_eq!(unit.last(), data);
  /// ```
  pub fn push(&mut self, value: &[u8]) -> Result<(), Error> {
    if self.len() > 9999 {
      return Err(Error::ArrayFull);
    }

    if self.seek + value.len() > self.mmap.len() {
      return Err(Error::FileFull);
    }

    self.mmap[self.seek..self.seek + value.len()].clone_from_slice(value);
    self.set_seek(value.len());

    Ok(())
  }

  /// Get `index` from vector
  ///
  /// ## Examples
  /// ```ignore
  /// let mut unit = MMAVUnit::new("test-0/0", 4000000, 80008);
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// unit.push(data).unwrap_or_default();
  /// assert_eq!(unit.get(0), data);
  /// ```
  pub fn get(&self, index: usize) -> Result<Vec<u8>, Error> {
    if self.seek_index == 8 {
      return Err(Error::ArrayEmpty);
    }

    if index > 9999 {
      return Err(Error::IndexOutOfRange);
    }

    if index > self.len() - 1 {
      return Err(Error::IndexOutOfBounds);
    }

    let i = 8 * index + 8;

    let start = u32::from_ne_bytes(self.mmap[i..i + 4].try_into()?) as usize;
    let end = u32::from_ne_bytes(self.mmap[i + 4..i + 8].try_into()?) as usize;

    if start < self.data_start_index || start > self.mmap.len() {
      return Err(Error::IndexOutOfRange);
    }

    if end < self.data_start_index || end > self.mmap.len() {
      return Err(Error::IndexOutOfRange);
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
