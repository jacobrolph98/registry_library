#[cfg(test)]
mod test_support {
    #[derive(Debug, Clone, PartialEq)]
    pub struct TestStats {
        pub id: String,
        pub cost: f32
    }

    pub fn def(name: &str, value: f32) -> TestStats {
        TestStats { 
            id: name.to_string(),
            cost: value
         }
    }
}

#[cfg(test)]
mod interner_tests {
    use super::test_support::TestStats;
    use crate::interner::Interner;

    #[test]
    fn duplicate_id_test() {
        let mut interner = Interner::<TestStats>::default();
        let a = interner.intern("base:sword");
        let b = interner.intern("base:sword");
        assert_eq!(a, b);
    }

    #[test]
    fn differing_id_test() {
        let mut interner = Interner::<TestStats>::default();
        let a = interner.intern("base:sword");
        let b = interner.intern("base:shield");
        assert_ne!(a, b);
    }

    #[test]
    fn resolve_test() {
        let mut interner = Interner::<TestStats>::default();
        let id = interner.intern("base:sword");
        assert_eq!(interner.resolve(id), "base:sword");
    }

    #[test]
    fn get_existing_test() {
        let mut interner = Interner::<TestStats>::default();
        interner.intern("base:sword");
        assert_eq!(interner.get_existing("base:sword"), interner.get_existing("base:sword"));
        assert!(interner.get_existing("base:sword").is_some());
    }

    #[test]
    fn get_not_existing_test() {
        let interner = Interner::<TestStats>::default();
        assert!(interner.get_existing("base:nonexistent").is_none());
    }
}

#[cfg(test)]
mod registry_tests {
    use super::test_support::*;
    use crate::{interner::Interner, registry::Registry};

    #[test]
    fn register_get_test() {
        let mut interner = Interner::<TestStats>::default();
        let mut registry = Registry::<TestStats>::default();
        let id = interner.intern("base:sword");

        registry.register(id, def("base:sword", 10.)).unwrap();

        assert_eq!(registry.get(id).unwrap().cost, 10.);
    }

    #[test]
    fn register_fails_on_duplicate() {
        let mut interner = Interner::<TestStats>::default();
        let mut registry = Registry::<TestStats>::default();
        let id = interner.intern("base:sword");

        registry.register(id, def("base:sword", 10.)).unwrap();
        let result = registry.register(id, def("base:sword", 99.));

        assert!(result.is_err());
        assert_eq!(registry.get(id).unwrap().cost, 10.);
    }

    #[test]
    fn register_or_override_test() {
        let mut interner = Interner::<TestStats>::default();
        let mut registry = Registry::<TestStats>::default();
        let id = interner.intern("base:sword");

        registry.register(id, def("base:sword", 10.)).unwrap();
        let overwritten = registry.register_or_override(id, def("base:sword", 99.));

        assert!(overwritten);
        assert_eq!(registry.get(id).unwrap().cost, 99.);
    }

    #[test]
    fn registering_out_of_order_test() {
        let mut interner = Interner::<TestStats>::default();
        let mut registry = Registry::<TestStats>::default();

        let a = interner.intern("a");
        let b = interner.intern("b");
        let c = interner.intern("c");

        registry.register(c, def("c", 3.)).unwrap();
        registry.register(a, def("a", 1.)).unwrap();

        assert_eq!(registry.get(a).unwrap().cost, 1.);
        assert_eq!(registry.get(c).unwrap().cost, 3.);
        assert!(registry.get(b).is_none());
    }
}