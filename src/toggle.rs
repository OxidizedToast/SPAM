// SPDX-License-Identifier: Apache-2.0
use crate::config_file;

pub fn toggle(){
    // due to returns a bool it sets it as the value
    let daemon_actived = config_file::check_file_contents();
    if daemon_actived == true{
        // make file read to false
        println!("Toggling S.P.A.M Off");
    } else if daemon_actived == false {
       // make file read to true 
        println!("Toggling S.P.A.M On");
    }
}
