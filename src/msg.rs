use std::any::{TypeId, Any};

impl Message {
    fn get_message_type_id(&self) -> TypeId {
        self.get_type_id()
    }

    pub fn cast<T: Message + Sized>(self: Box<Message>) -> Option<T> {
        if self.get_message_type_id() == TypeId::of::<T>() {
            let res = unsafe { *Box::from_raw(Box::into_raw(self) as *mut T) };
            Some(res)
        } else {
            None
        }
    }
}

pub trait Message : Any {

}

#[cfg(test)]
mod tests {
    use super::Message;

    #[derive(PartialEq, Eq, Debug)]
    struct Msg1;

    impl Message for Msg1 {

    }

    #[derive(PartialEq, Eq, Debug)]
    struct Msg2;

    impl Message for Msg2 {

    }

    #[test]
    fn cast_to_some() {
        let generic : Box<Message> = Box::new(Msg1);

        let casted = generic.cast::<Msg1>();

        assert_eq!(casted, Some(Msg1));
    }

    #[test]
    fn cast_to_none() {
        let generic : Box<Message> = Box::new(Msg1);

        let casted = generic.cast::<Msg2>();

        assert_eq!(casted, None);
    }
}