use crate::db::mmav::mmav_unit::MMAVUnit;
use crate::errors::MMAVError;

/// Memory Mapped Append-only Vector
///
/// This is a higher abstraction above MMAVUnit, that allows
/// using multiple MMAVUnits, which are loaded and unloaded on demand.
/// This ultimitely allows for a dynamically allocated append-only
/// vector, which uses memory mapped file io.
///
/// ## Examples
/// ```ignore
/// let mut mmav = MMAV::new(".db/test-0");
///
/// let data = b"{\"key\":\"value\"}";
/// mmav.push(data).unwrap_or_default();
/// assert_eq!(mmav.last(), data);
/// ```
#[allow(clippy::upper_case_acronyms)]
pub struct MMAV {
  id: String,
  index: usize,
  mmav_size: usize,
  indices: Vec<usize>,
  mmav_data_start_index: usize,
  unit_map: std::collections::HashMap<usize, MMAVUnit>,
}
impl MMAV {
  /// Memory Mapped Append-only Vector Constructor
  ///
  /// Creates a MMAV with given `id`
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav.last(), data);
  /// ```
  pub fn new(id: &str) -> Self {
    let mut index = Default::default();

    let paths = std::fs::read_dir(id).unwrap_or_else(|_| {
      std::fs::create_dir(id).unwrap_or_default();
      std::fs::read_dir(id).unwrap()
    });

    let mut indices = vec![];

    for path in paths {
      path
        .unwrap()
        .file_name()
        .into_string()
        .unwrap_or_default()
        .parse::<usize>()
        .map(|x| indices.push(x))
        .unwrap_or_default();
    }

    indices.sort_unstable();

    if indices.is_empty() {
      indices.push(index);
    }

    let mut unit_map = Default::default();
    let mmav_size = 14_580_008;
    let mmav_data_start_index = 80008;

    if let Some(x) = indices.last() {
      index = *x;
      MMAV::load_unchecked(
        id,
        &mut unit_map,
        *x,
        mmav_size,
        mmav_data_start_index,
      );
    };

    if indices.len() > 1 {
      let x = indices[indices.len() - 2..indices.len() - 1][0];
      MMAV::load_unchecked(
        id,
        &mut unit_map,
        x,
        mmav_size,
        mmav_data_start_index,
      );
    }

    Self {
      index,
      unit_map,
      indices,
      mmav_size,
      mmav_data_start_index,
      id: id.to_owned(),
    }
  }

  /// Computes the closest `index` in `array`
  ///
  /// ## Constraints
  /// `arr` needs to be sorted in ascending order
  ///
  /// ## Examples
  /// ```ignore
  /// assert_eq!(MMAV::bisect_left(5, vec![0, 2, 4, 6, 8, 10]), 4);
  /// ```
  fn bisect_left(index: usize, arr: &[usize]) -> usize {
    if arr.len() == 1 {
      return arr[0];
    }

    let middle = arr.len() / 2;
    let middle_item = arr[middle];

    if index < middle_item {
      return MMAV::bisect_left(index, &arr[..middle]);
    }

    if index > middle_item {
      return MMAV::bisect_left(index, &arr[middle..]);
    }

    index
  }

  /// Load unit with `id`, `size` and `data_start_index` into `unit_map`
  /// with `index` as key, without checking `self.indices`
  ///
  /// ## Examples
  /// ```ignore
  /// let mut unit_map = Default::default();
  /// MMAV::load_unchecked("test-0", &mut unit_map, 0, 4000000, 80008);
  /// ```
  fn load_unchecked(
    id: &str,
    unit_map: &mut std::collections::HashMap<usize, MMAVUnit>,
    index: usize,
    size: usize,
    data_start_index: usize,
  ) -> usize {
    if unit_map.contains_key(&index) {
      return unit_map[&index].len();
    }

    let unit = MMAVUnit::new(&format!("{id}/{index}"), size, data_start_index);
    let result = unit.len();

    unit_map.insert(index, unit);

    result
  }

  /// Load unit that contains `index`
  ///
  /// ## Examples
  /// ```ignore
  /// self.load(0);
  /// ```
  fn load(&mut self, index: usize) -> usize {
    MMAV::load_unchecked(
      &self.id,
      &mut self.unit_map,
      MMAV::bisect_left(index, &self.indices),
      self.mmav_size,
      self.mmav_data_start_index,
    )
  }

  /// Unload all units except the last `keep` number of units
  ///
  /// ## Examples
  /// ```ignore
  /// self.unload(1);
  /// ```
  fn unload(&mut self, keep: usize) {
    if keep > self.indices.len() {
      panic!("index out of range");
    }

    let mut to_remove = vec![];
    for key in self.unit_map.keys() {
      if !self.indices[self.indices.len() - keep..].contains(key) {
        to_remove.push(*key);
      }
    }

    for key in to_remove {
      self.unit_map.remove(&key);
    }
  }

  /// Expand vector
  ///
  /// ## Examples
  /// ```ignore
  /// self.expand();
  /// ```
  fn expand(&mut self) {
    self.unload(1);

    self.index += self.unit_map[&self.index].len();
    self.indices.push(self.index);

    if let Some(x) = self.indices.last() {
      MMAV::load_unchecked(
        &self.id,
        &mut self.unit_map,
        *x,
        self.mmav_size,
        self.mmav_data_start_index,
      );
    }
  }

  /// Push `value` to vector
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav.last(), data);
  /// ```
  pub fn push(&mut self, value: &[u8]) {
    self.unit_map.get_mut(&self.index).unwrap().push(value).unwrap_or_else(
      |error| match error {
        MMAVError::ArrayFull => {
          self.expand();
          self.push(value);
        }
        MMAVError::FileFull => {
          self.expand();
          self.push(value);
        }
        _ => (),
      },
    );
  }

  /// Get `value` at `index` immutably
  ///
  /// ## Examples
  /// ```ignore
  /// self._get(0)
  /// ```
  fn _get(&self, index: usize) -> Vec<u8> {
    if self.len() == 0 || index > self.len() - 1 {
      return Default::default();
    }

    let closest = MMAV::bisect_left(index, &self.indices);

    self.unit_map[&closest].get(index - closest).unwrap_or_default()
  }

  /// Get `value` at `index`
  ///
  /// May load data from disk, if it is not in-memory,
  /// as such it is mutable, even though a get normaly is immutable.
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav.get(0), data);
  /// ```
  pub fn get(&mut self, index: usize) -> Vec<u8> {
    self.load(index);

    self._get(index)
  }

  /// Get last item in vector
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav.last(), data);
  /// ```
  pub fn last(&self) -> Vec<u8> {
    self
      .indices
      .last()
      .map(|x| self.unit_map.get(x).map(|y| y.last()))
      .unwrap_or_default()
      .unwrap_or_default()
  }

  /// Get length of vector
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav.len(), 1);
  /// ```
  pub fn len(&self) -> usize {
    return self
      .indices
      .last()
      .map(|x| self.index + self.unit_map[x].len())
      .unwrap_or_default();
  }

  /// Get range from `start` to `end` immutably
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav._range(0, 1), vec![data, data]);
  /// ```
  fn _range(&self, start: usize, end: usize) -> Vec<Vec<u8>> {
    let mut result = vec![];

    if end > self.len() {
      return result;
    }

    for i in start..=end {
      let item = self._get(i);

      if !item.is_empty() {
        result.push(item);
      }
    }

    result
  }

  /// Get range from `start` to `end`
  ///
  /// May load data from disk, if it is not in-memory,
  /// as such it is mutable, even though a range scan normaly is
  /// immutable.
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav.range(0, 1), vec![data, data]);
  /// ```
  pub fn range(&mut self, start: usize, end: usize) -> Vec<Vec<u8>> {
    let start_size = self.load(start);
    let end_size = self.load(end);

    let size = (|| {
      if start_size != end_size {
        return start_size + end_size;
      }
      start_size
    })();

    if std::cmp::max(start, end) - std::cmp::min(start, end) > size {
      return self._range(start, start + size - 1);
    }

    self._range(start, end)
  }

  /// Get last `limit` number of items from vector
  ///
  /// May load data from disk, if it is not in-memory,
  /// as such it is mutable, even though a get last item normaly is
  /// immutable.
  ///
  /// ## Examples
  /// ```ignore
  /// let mut mmav = MMAV::new(".db/test-0");
  ///
  /// let data = b"{\"key\":\"value\"}";
  /// mmav.push(data).unwrap_or_default();
  /// mmav.push(data).unwrap_or_default();
  /// assert_eq!(mmav.last_limit(2), vec![data, data]);
  /// ```
  pub fn last_limit(&mut self, limit: usize) -> Vec<Vec<u8>> {
    if self.len() == 0 {
      return Default::default();
    }

    if limit > self.len() {
      return self.range(0, self.len() - 1);
    }

    self.range(self.len() - limit, self.len() - 1)
  }
}
