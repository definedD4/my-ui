use std::any::{TypeId, Any};

pub struct AnyMessage {
    type_id: TypeId,
    inner: Box<Message + 'static>,
}

impl AnyMessage {
    pub fn new<T: Message>(msg: T) -> AnyMessage {
        AnyMessage {
            type_id: msg.get_type_id(),
            inner: Box::new(msg),
        }
    }

    pub fn cast<T: Message>(self) -> Option<T> {
        let AnyMessage { type_id, inner } = self;
        if type_id == TypeId::of::<T>() {           
            Some(*inner.dowcast_ref::<T>())
        } else {
            None
        }
    }
}

pub trait Message : Any, Clone {

}
