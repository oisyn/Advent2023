use std::iter::StepBy;

pub struct FieldView<'a, T> {
    data: &'a [T],
    width: usize,
    height: usize,
    stride: usize,
}

impl<'a, T> FieldView<'a, T> {
    pub fn new(data: &'a [T], width: usize, stride: usize, height: usize) -> Self {
        Self {
            data: &data[..height * stride - stride + width],
            width,
            height,
            stride,
        }
    }

    pub fn offset(&self, x: usize, y: usize) -> usize {
        y * self.stride + x
    }

    pub fn data(&self) -> &[T] {
        self.data
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.offset(x, y)]
    }

    pub fn row(&self, index: usize) -> &[T] {
        let o = index * self.stride;
        &self.data[o..o + self.width]
    }

    pub fn col(&self, index: usize) -> FieldColumn<'a, T> {
        FieldColumn {
            data: &self.data[index..],
            stride: self.stride,
            height: self.height,
        }
    }

    pub fn rows(&self) -> FieldRows<'a, T> {
        FieldRows(self.clone())
    }

    pub fn cols(&self) -> FieldCols<'a, T> {
        FieldCols(self.clone())
    }
}

impl<'a, T> Clone for FieldView<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

#[derive(Clone)]
pub struct FieldRows<'a, T>(FieldView<'a, T>);

impl<'a, T> ExactSizeIterator for FieldRows<'a, T> {
    fn len(&self) -> usize {
        self.0.height
    }
}

impl<'a, T> Iterator for FieldRows<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.height == 0 {
            None
        } else {
            self.0.height -= 1;
            let d = &self.0.data[0..self.0.width];
            self.0.data = &self.0.data[self.0.stride.min(self.0.data.len())..];
            Some(d)
        }
    }
}

impl<'a, T> DoubleEndedIterator for FieldRows<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0.height == 0 {
            None
        } else {
            self.0.height -= 1;
            let d = &self.0.data[self.0.data.len() - self.0.width..];
            self.0.data = &self.0.data[..self.0.data.len().max(self.0.stride) - self.0.stride];
            Some(d)
        }
    }
}

#[derive(Clone)]
pub struct FieldCols<'a, T>(FieldView<'a, T>);

impl<'a, T> ExactSizeIterator for FieldCols<'a, T> {
    fn len(&self) -> usize {
        self.0.width
    }
}

impl<'a, T> Iterator for FieldCols<'a, T> {
    type Item = FieldColumn<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.width == 0 {
            None
        } else {
            self.0.width -= 1;
            let d = self.0.data;
            self.0.data = &self.0.data[1..];
            Some(FieldColumn {
                data: d,
                stride: self.0.stride,
                height: self.0.height,
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for FieldCols<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.0.width == 0 {
            None
        } else {
            self.0.width -= 1;
            let d = &self.0.data[self.0.width..];
            Some(FieldColumn {
                data: d,
                stride: self.0.stride,
                height: self.0.height,
            })
        }
    }
}

pub struct FieldColumn<'a, T> {
    data: &'a [T],
    stride: usize,
    height: usize,
}

impl<'a, T> FieldColumn<'a, T> {
    pub fn len(&self) -> usize {
        self.height
    }
}

impl<'a, T> Clone for FieldColumn<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> std::ops::Index<usize> for FieldColumn<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.stride]
    }
}

impl<'a, T> IntoIterator for FieldColumn<'a, T> {
    type Item = &'a T;
    type IntoIter = StepBy<std::slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().step_by(self.stride)
    }
}

impl<'a, T: PartialEq> PartialEq for FieldColumn<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.clone().into_iter().eq(other.clone().into_iter())
    }
}

impl<'a, T: Eq> Eq for FieldColumn<'a, T> {}

impl<'a, T: PartialOrd> PartialOrd for FieldColumn<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.clone()
            .into_iter()
            .partial_cmp(other.clone().into_iter())
    }
}

impl<'a, T: Ord> Ord for FieldColumn<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.clone().into_iter().cmp(other.clone().into_iter())
    }
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for FieldColumn<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone().into_iter()).finish()
    }
}
