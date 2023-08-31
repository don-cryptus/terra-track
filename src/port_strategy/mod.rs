mod range_iterator;

use range_iterator::RangeIterator;

#[derive(Debug)]
pub enum PortStrategy {
    Serial(SerialRange),
}

impl PortStrategy {
    pub fn order(&self) -> Vec<u16> {
        match self {
            PortStrategy::Serial(range) => range.generate(),
        }
    }
}

trait RangeOrder {
    fn generate(&self) -> Vec<u16>;
}

#[derive(Debug)]
pub struct SerialRange {
    pub(crate) start: u16,
    pub(crate) end: u16,
}

impl RangeOrder for SerialRange {
    fn generate(&self) -> Vec<u16> {
        (self.start..self.end).collect()
    }
}

#[derive(Debug)]
pub struct RandomRange {
    start: u16,
    end: u16,
}

impl RangeOrder for RandomRange {
    fn generate(&self) -> Vec<u16> {
        RangeIterator::new(self.start.into(), self.end.into()).collect()
    }
}
