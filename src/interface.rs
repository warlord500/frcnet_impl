#[macro_use]
extern crate lazy_static;

use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use super::frcNetImpl;


lazy_static! {
    ///I decided that I couldnt trust 
    /// external code to manage pointer access.
    /// maybe not the best desicion but pretty much all of 
    /// this code relies on having access to threadinfo
    /// in trade of allowing c code to access pointer.
    static netThreadData : AtomicPtr<frcNetImpl> = AtomicPtr::new(0 as *mut frcNetImpl);
}

/// start the thread that comminicates with driverStation
/// 
/// this the only method that is potentially thread unsafe. 
extern  fn start_thread(team_id: i32) -> bool {
    use std::thread;

    if !createdThread() { //if the program hasnt created a thread.
         let threadInfo = frcNetImpl::new();
         
         let _ = thread::spawn(move || {
             let mut threadInfo = threadInfo; //create variable inside thread
             unsafe {
                 let threadRef = &mut threadInfo as *mut frcNetImpl;
                 netThreadData.store(threadRef,Ordering::Relaxed); 
                 //set variable so that code can test correctly of thread creation.
                }
             threadInfo.execute_thread(team_id);
             });
         true
     } else {
         false
     }  
}
          
            
    

extern fn observe_automonus() -> () {
 }
extern  fn observe_telop() -> () {
 }
extern fn observe_test() -> () {
 }
extern fn observeUserProgramDisabled() -> () {
}

extern fn getJoystickData()  
{ 
}
///determine if thread has been created yet!
extern fn createdThread() -> bool{
 netThreadData.load(Ordering::Relaxed) != (0 as *mut frcNetImpl) 
} 

