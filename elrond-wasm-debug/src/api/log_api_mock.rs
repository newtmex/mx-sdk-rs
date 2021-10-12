use elrond_wasm::{
    api::{Handle, LogApi},
    types::{
        managed_vec_of_buffers_to_arg_buffer, ArgBuffer, ManagedBuffer, ManagedType, ManagedVec,
    },
};

use crate::tx_mock::{DebugApi, TxLog};

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl LogApi for DebugApi {
    fn write_event_log(&self, topics_buffer: &ArgBuffer, data: &[u8]) {
        let arg_data_buffer = topics_buffer.arg_data();
        let arg_data_lengths = topics_buffer.arg_lengths();

        let mut current_index = 0;
        let mut topics = Vec::new();

        // we already processed the first data arg, so we skip it
        for arg_len in arg_data_lengths.iter() {
            let topic = arg_data_buffer[current_index..(current_index + arg_len)].to_vec();
            topics.push(topic);

            current_index += arg_len;
        }

        let mut tx_output_cell = self.output_borrow_mut();
        tx_output_cell.result.result_logs.push(TxLog {
            address: self.input_ref().to.clone(),
            endpoint: self.input_ref().func_name.clone(),
            topics,
            data: data.to_vec(),
        });
    }

    fn write_legacy_log(&self, topics: &[[u8; 32]], data: &[u8]) {
        let topics_vec = topics.iter().map(|array| array.to_vec()).collect();

        let mut tx_output_cell = self.output_borrow_mut();
        tx_output_cell.result.result_logs.push(TxLog {
            address: self.input_ref().to.clone(),
            endpoint: self.input_ref().func_name.clone(),
            topics: topics_vec,
            data: data.to_vec(),
        });
    }

    fn managed_write_log(&self, topics_handle: Handle, data_handle: Handle) {
        let topics = ManagedVec::from_raw_handle(self.clone(), topics_handle);
        let topics_arg_buffer = managed_vec_of_buffers_to_arg_buffer(&topics);
        let data = ManagedBuffer::from_raw_handle(self.clone(), data_handle);
        self.write_event_log(&topics_arg_buffer, data.to_boxed_bytes().as_slice());
    }
}
