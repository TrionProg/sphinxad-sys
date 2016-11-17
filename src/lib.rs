extern crate libc;
use libc::{c_char,c_void};

///Device
#[repr(C)]
pub struct ad_rec_t(c_void);

///Uses for ad_open function
pub const DEFAULT_SAMPLES_PER_SEC:i32 =	16000;

///Return codes
pub enum ReturnCode{
    Ok=0,
    ErrGen=-1,
    ErrNotOpen=-2,
    ErrWave=-3,
}

///EOF Return code
pub const AD_EOF:i32 =	      	-1;

#[link(name = "sphinxad")]
extern {
    ///Open a specific audio device with a given name and sampling rate.
    ///
    ///The device is opened in non-blocking mode and placed in idle state.
    ///The return value to be used as the first argument to
    ///other recording functions.
    /// # Safety
    ///
    ///Return pointer to read-only ad_rec_t structure if successful, NULL
    ///otherwise.
    pub fn ad_open_dev(device_name: *const c_char, samples_per_sec: i32) -> *const ad_rec_t;

    ///Open the default audio device with a given sampling rate.
    ///
    ///The device is opened in non-blocking mode and placed in idle state.
    ///The return value to be used as the first argument to
    ///other recording functions.
    /// # Safety
    ///
    ///Return pointer to read-only ad_rec_t structure if successful, NULL
    ///otherwise.
    pub fn ad_open_sps(samples_per_sec:i32) -> *const ad_rec_t;

    ///Open the default audio device.
    ///
    ///The device is opened in non-blocking mode and placed in idle state.
    ///The return value to be used as the first argument to
    ///other recording functions.
    /// # Safety
    ///
    ///Return pointer to read-only ad_rec_t structure if successful, NULL
    ///otherwise.
    pub fn ad_open() -> *const ad_rec_t;

    ///Start audio recording.
    ///
    /// # Safety
    ///
    ///Return 0(ReturnCode::Ok) if successful, <0 otherwise
    pub fn ad_start_rec(dev: *const ad_rec_t) -> i32;

    ///Stop audio recording.
    ///
    /// # Safety
    ///
    ///Return 0(ReturnCode::Ok) if successful, <0 otherwise
    pub fn ad_stop_rec(dev: *const ad_rec_t) -> i32;

    ///Close the recording device.
    ///
    /// # Safety
    ///
    ///Return 0(ReturnCode::Ok) if successful, <0 otherwise
    pub fn ad_close(dev: *const ad_rec_t) -> i32;

    ///Read next block of audio samples while recording; read upto max samples into buf.
    ///
    /// # Safety
    ///
    ///Return # samples actually read (could be 0 since non-blocking); AD_EOF if not
    ///recording and no more samples remaining to be read from most recent recording.
    pub fn ad_read(dev: *const ad_rec_t, buf:*mut i16, max:i32) -> i32;
}
