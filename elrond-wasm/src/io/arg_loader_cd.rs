
use crate::*;
use crate::call_data::*;

pub struct CallDataArgLoader<'a, SE>
where
    SE: SignalError
{
    deser: CallDataDeserializer<'a>,
    signal_error: SE,
}

impl<'a, SE> CallDataArgLoader<'a, SE>
where
    SE: SignalError
{
    pub fn new(deser: CallDataDeserializer<'a>, signal_error: SE) -> Self {
        CallDataArgLoader {
            deser,
            signal_error,
        }
    }
}

impl<'a, SE> SignalError for CallDataArgLoader<'a, SE>
where
    SE: SignalError
{
    #[inline]
    fn signal_error(&self, message: &[u8]) -> ! {
        self.signal_error.signal_error(message)
    }
}

impl<'a, SE> DynArgInput<Vec<u8>> for CallDataArgLoader<'a, SE>
where
    SE: SignalError
{
    #[inline]
    fn has_next(&self) -> bool {
        self.deser.has_next()
    }

    fn next_arg_input(&mut self) -> Option<Vec<u8>> {
        match self.deser.next_argument() {
            Ok(Some(arg_bytes)) => Some(arg_bytes),
            Ok(None) => None,
            Err(sc_err) => self.signal_error(sc_err.as_bytes())
        }
    }
}
