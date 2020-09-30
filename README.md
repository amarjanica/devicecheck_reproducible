# Minimal Reproducible Example - [SO Question](https://stackoverflow.com/questions/64141924/how-to-get-past-unable-to-verify-authorization-token) 

## Problem

Cannot verify with Apple Device Check API.

## Prerequisites

-  Generate a DeviceCheck Key on Apple developer portal. 
Go to https://developer.apple.com/account/ios/authkey/create to create a key, and check DeviceCheck in the Key Services list.
You should get a `pem certificate` and `key_id`.
-  To get `team_id`, login to [Apple's developer account](https://developer.apple.com/account/#/welcome), then click on Account and Membership.


## Run
`cargo run`
