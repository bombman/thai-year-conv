/// Trait สำหรับการแปลงปี
pub trait YearConversion {
    /// แปลงปีจาก ค.ศ. เป็น พ.ศ.
    ///
    /// # ตัวอย่าง
    ///
    /// ```
    /// use thai_year_conv::Year;
    /// let year = Year::new(2021);
    /// assert_eq!(year.to_buddhist_year(), 2564);
    /// ```
    fn to_buddhist_year(&self) -> i32;

    /// แปลงปีจาก พ.ศ. เป็น ค.ศ.
    ///
    /// # ตัวอย่าง
    ///
    /// ```
    /// use thai_year_conv::Year;
    /// let year = Year::new(2564);
    /// assert_eq!(year.to_christian_year(), 2021);
    /// ```
    fn to_christian_year(&self) -> i32;
}

/// Struct สำหรับเก็บปี
pub struct Year {
    year: i32,
}

impl Year {
    /// สร้าง `Year` ใหม่
    ///
    /// # ตัวอย่าง
    ///
    /// ```
    /// use thai_year_conv::Year;
    /// let year = Year::new(2021);
    /// ```
    pub fn new(year: i32) -> Self {
        Self { year }
    }
}

impl YearConversion for Year {
    fn to_buddhist_year(&self) -> i32 {
        self.year + 543
    }

    fn to_christian_year(&self) -> i32 {
        self.year - 543
    }
}
