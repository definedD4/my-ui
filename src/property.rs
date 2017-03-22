#[derive(Copy, Clone)]
pub enum AccessType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl AccessType {
    pub fn read_enabled(&self) -> bool {
        match *self {
            AccessType::ReadOnly | AccessType::ReadWrite => true,
            AccessType::WriteOnly => false,
        }
    }

    pub fn write_enabled(&self) -> bool {
        match *self {
            AccessType::WriteOnly | AccessType::ReadWrite => true,
            AccessType::ReadOnly => false,
        }
    }

    pub fn read_write_enabled(&self) -> bool {
        match *self {
            AccessType::ReadWrite => true,
            AccessType::ReadOnly | AccessType::WriteOnly => false,
        }
    }

    pub fn suits(&self, other: AccessType) -> bool{
        match *self {
            AccessType::ReadOnly => other.read_enabled(),
            AccessType::WriteOnly => other.write_enabled(),
            AccessType::ReadWrite => other.read_write_enabled(),
        }
    }
}

pub struct Property<T: Copy> {
    name: String,
    access_type: AccessType,
    source: Box<ValueSource<T>>,
}

pub trait ValueSource<T: Copy> {
    fn access_type(&self) -> AccessType;
    fn get(&self) -> T;
    fn set(&self, value: T);
}

impl<T: Copy> Property<T> {
    pub fn new<S: Into<String>>(name: S, access_type: AccessType, source: Box<ValueSource<T>>) -> Result<Property<T>, String> {
        if !access_type.suits(source.access_type()) {
            Err("Unsiutable value source provided".to_owned())
        } else {
            Ok(Property { 
                name: name.into(),
                access_type:access_type,
                source: source
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn access_type(&self) -> AccessType {
        self.access_type
    }

    pub fn get(&self) -> T {
        if !self.access_type.read_enabled() {
            panic!("Calling get on property with disabled read");
        }
        self.source.get()
    }

    pub fn set(&self, value: T) {
        if !self.access_type.write_enabled() {
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
        fn access_type(&self) -> AccessType { AccessType::ReadWrite }

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
        let prop = Property::new("foo", AccessType::ReadWrite, Box::new(source::Variable::new(0))).unwrap();

        assert_eq!(prop.name(), "foo");
    }

    #[test]
    fn variable_property() {
        let prop = Property::new("foo", AccessType::ReadWrite, Box::new(source::Variable::new(0))).unwrap();

        assert_eq!(prop.get(), 0);

        prop.set(3);

        assert_eq!(prop.get(), 3);
    }

    #[test]
    #[should_panic]
    fn set_read_only_property() {
        let prop = Property::new("foo", AccessType::ReadOnly, Box::new(source::Variable::new(0))).unwrap();

        prop.set(3);
    }

    #[test]
    #[should_panic]
    fn read_write_only_property() {
        let prop = Property::new("foo", AccessType::WriteOnly, Box::new(source::Variable::new(0))).unwrap();

        prop.get();
    }
}