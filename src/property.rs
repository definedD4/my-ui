#[derive(Copy, Clone)]
pub enum PropertyTraits {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl PropertyTraits {
    pub fn read_enabled(&self) -> bool {
        match *self {
            PropertyTraits::ReadOnly | PropertyTraits::ReadWrite => true,
            PropertyTraits::WriteOnly => false,
        }
    }

    pub fn write_enabled(&self) -> bool {
        match *self {
            PropertyTraits::WriteOnly | PropertyTraits::ReadWrite => true,
            PropertyTraits::ReadOnly => false,
        }
    }

    pub fn read_write_enabled(&self) -> bool {
        match *self {
            PropertyTraits::ReadWrite => true,
            PropertyTraits::ReadOnly | PropertyTraits::WriteOnly => false,
        }
    }

    pub fn suits(&self, other: PropertyTraits) -> bool{
        match *self {
            PropertyTraits::ReadOnly => other.read_enabled(),
            PropertyTraits::WriteOnly => other.write_enabled(),
            PropertyTraits::ReadWrite => other.read_write_enabled(),
        }
    }
}

pub struct Property<T> {
    name: String,
    traits: PropertyTraits,
    source: Box<ValueSource<T>>,
}

pub trait ValueSource<T> {
    fn traits(&self) -> PropertyTraits;
    fn get(&self) -> T;
    fn set(&self, value: T);
}

impl<T> Property<T> {
    pub fn new<S: Into<String>>(name: S, traits: PropertyTraits, source: Box<ValueSource<T>>) -> Result<Property<T>, String> {
        if !traits.suits(source.traits()) {
            Err("Unsiutable value source provided".to_owned())
        } else {
            Ok(Property { 
                name: name.into(),
                traits:traits,
                source: source
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn traits(&self) -> PropertyTraits {
        self.traits
    }

    pub fn get(&self) -> T {
        if !self.traits.read_enabled() {
            panic!("Calling get on property with disabled read");
        }
        self.source.get()
    }

    pub fn set(&self, value: T) {
        if !self.traits.write_enabled() {
            panic!("Calling set on property with disabled write");
        }
        self.source.set(value)
    }
}

pub mod source {
    use super::*;

    use std::cell::Cell;

    pub struct Variable<T> {
        data: Cell<T>,
    }

    impl<T> Variable<T> where T: Copy {
        pub fn new(default: T) -> Variable<T> {
            Variable {
                data: Cell::new(default)
            }
        }
    }

    impl<T> ValueSource<T> for Variable<T> where T: Copy {
        fn traits(&self) -> PropertyTraits { PropertyTraits::ReadWrite }

        fn get(&self) -> T {
            self.data.get()
        }

        fn set(&self, value: T) {
            self.data.set(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_name_get() {
        let prop = Property::new("foo", PropertyTraits::ReadWrite, Box::new(source::Variable::new(0))).unwrap();

        assert_eq!(prop.name(), "foo");
    }

    #[test]
    fn variable_property() {
        let prop = Property::new("foo", PropertyTraits::ReadWrite, Box::new(source::Variable::new(0))).unwrap();

        assert_eq!(prop.get(), 0);

        prop.set(3);

        assert_eq!(prop.get(), 3);
    }
}