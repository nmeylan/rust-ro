use std::collections::VecDeque;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;

#[derive(Default)]
pub struct VecWithRecycledIndex<T> {
    elements: Vec<T>,
    ids_pool: VecDeque<u32>,
    sequence: AtomicU32,
}

impl <T>VecWithRecycledIndex<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            ids_pool: Default::default(),
            sequence: Default::default(),
        }
    }
    pub fn get_elements(&self) -> &Vec<T> {
        &self.elements
    }
    pub fn get_elements_mut(&mut self) -> &mut Vec<T> {
        &mut self.elements
    }
    #[inline]
    pub fn get_free_index(&mut self) -> usize {
        if let Some(recycled_id) = self.ids_pool.pop_front() {
            recycled_id as usize
        } else {
            self.sequence.fetch_add(1, SeqCst) as usize
        }
    }

    #[inline]
    pub fn push(&mut self, element: T) -> usize {
        let index = self.get_free_index();
        self.elements.insert(index, element);
        index
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: T) -> usize {
        self.elements.insert(index, element);
        index
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        self.ids_pool.push_back(index as u32);
        self.elements.remove(index)
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }
}