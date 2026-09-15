//! S240 — allocateur compteur global de l'hôte, pour mesurer I-06 sur la pile graphique.
//!
//! Il enveloppe `std::alloc::System` et compte, sans aucune dépendance : nombre d'allocations,
//! octets demandés, nombre de libérations. Les compteurs sont atomiques en ordre **relâché** :
//! chaque relevé est pris entre deux bornes de phase du même fil, et aucune autre donnée n'est
//! publiée par eux. Voir `docs/validation/ALLOCATIONS-HOTE-S240.md`.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static FREES: AtomicU64 = AtomicU64::new(0);

pub struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size() as u64, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size() as u64, Relaxed);
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        FREES.fetch_add(1, Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // Une réallocation est un mouvement de mémoire : elle compte comme une allocation, et
        // seule la croissance compte en octets.
        ALLOCS.fetch_add(1, Relaxed);
        BYTES.fetch_add(new_size.saturating_sub(layout.size()) as u64, Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// Relevé instantané des compteurs. Copié, jamais alloué.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Mark {
    pub allocs: u64,
    pub bytes: u64,
    pub frees: u64,
}

pub fn mark() -> Mark {
    Mark {
        allocs: ALLOCS.load(Relaxed),
        bytes: BYTES.load(Relaxed),
        frees: FREES.load(Relaxed),
    }
}

impl Mark {
    /// Ce qui s'est passé depuis `earlier`.
    pub fn since(self, earlier: Mark) -> Mark {
        Mark {
            allocs: self.allocs - earlier.allocs,
            bytes: self.bytes - earlier.bytes,
            frees: self.frees - earlier.frees,
        }
    }
}

/// Une phase de la boucle d'image : allocations et octets relevés image par image.
/// Les tampons sont **réservés une fois** — un relevé qui alloue fausserait sa propre mesure.
pub struct Phase {
    pub name: &'static str,
    pub allocs: Vec<f64>,
    pub bytes: Vec<f64>,
}

impl Phase {
    pub fn new(name: &'static str, capacity: usize) -> Self {
        Self { name, allocs: Vec::with_capacity(capacity), bytes: Vec::with_capacity(capacity) }
    }
    pub fn push(&mut self, delta: Mark) {
        self.allocs.push(delta.allocs as f64);
        self.bytes.push(delta.bytes as f64);
    }
}
