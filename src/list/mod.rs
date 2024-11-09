#[derive(Debug, Clone)]
pub struct List<T> {
    elements: Vec<T>,
}

impl<T: PartialEq> List<T> {
    pub fn new() -> Self {
        List {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // Bug 1
    pub fn remove(&mut self, item: T) {
        let index: Option<usize> = self.elements.iter().position(|x: &T| *x == item);
        match index {
            Some(i) => {
                self.elements.remove(i);
            }
            _ => {
                // Do nothing
            }
        }
    }

    // Bug 2
    // pub fn remove(&mut self, _item: T) {
    //     self.elements = Vec::new();
    // }

    // fixed version of remove
    // pub fn remove(&mut self, item: T) {
    //     while let Some(i) = self.elements.iter().position(|x: &T| *x == item) {
    //         self.elements.remove(i);
    //     }
    //
    //     // oder:
    //     // self.elements.retain(|x| *x != item);
    // }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // find first index of element
    pub fn find(&self, item: T) -> Option<usize> {
        self.elements.iter().position(|x: &T| *x == item)
    }
}
