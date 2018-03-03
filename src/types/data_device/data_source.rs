//! TODO Documentation

use std::marker::PhantomData;

use wlroots_sys::{wl_data_device_manager_dnd_action, wlr_data_offer, wlr_data_source};

/// An offering of data
#[derive(Debug)]
pub struct DataOffer<'source> {
    offer: *mut wlr_data_offer,
    phantom: PhantomData<&'source DataSource>
}

impl<'source> DataOffer<'source> {
    pub fn actions(&self) -> u32 {
        unsafe { (*self.offer).actions }
    }

    pub fn preferred_action(&self) -> wl_data_device_manager_dnd_action {
        unsafe { (*self.offer).preferred_action }
    }

    pub fn in_ask(&self) -> bool {
        unsafe { (*self.offer).in_ask }
    }
}

#[derive(Debug)]
pub struct DataSource {
    source: *mut wlr_data_source
}

// TODO Be able to set the function pointers?

impl DataSource {
    /// Get the data offer from this source.
    pub fn offer<'source>(&'source mut self) -> DataOffer<'source> {
        unsafe {
            DataOffer { offer: (*self.source).offer,
                        phantom: PhantomData }
        }
    }

    // TODO Mime types

    pub fn action(&self) -> i32 {
        unsafe { (*self.source).actions }
    }

    pub fn accepted(&self) -> bool {
        unsafe { (*self.source).accepted }
    }

    // TODO Seat client

    pub fn current_dnd_action(&self) -> wl_data_device_manager_dnd_action {
        unsafe { (*self.source).current_dnd_action }
    }

    pub fn compositor_action(&self) -> u32 {
        unsafe { (*self.source).compositor_action }
    }
}
