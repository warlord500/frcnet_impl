extern crate libc;
extern crate bindgen;
use super::frcNetImpl;

extern "C" {
    /// I am still trying to figure out how to make this function
    /// is safe.
     fn start_thread(team_id: i32) ->  *mut frcNetImpl {
    let mut thread_info = box::new(frcNetImpl::new());
    let rawPtr = *thread_info as *mut frcNetImpl;
    

    thread_info.execute_thread(team_id.clone());
    println!("start_thread");
        rawPtr
    }


}
