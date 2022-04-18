var searchIndex = JSON.parse('{\
"zkp_client":{"doc":"zkp_client includes modules and functions used to …","t":[6,17,17,17,17,0,5,5,5,5,5,0,0,5,5,5,5,5,5,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,17,17,3,11,11,11,11,11,11,11,5,11,11,12,11,11,11,11,12,11],"n":["Callback","MAX_SECRET_VAL","MAX_SEED_VAL","MAX_USERNAME_LEN","MIN_SEED_VAL","auth","connect","init_logger","init_zkp_flow","login_callback","print_errors","seed","user","check_status","create_auth_request","create_register_commits","make_request","prove_auth","register_user_with_server","Seed","borrow","borrow_mut","clone","clone_into","eq","fmt","from","into","ne","new","to_owned","try_from","try_into","type_id","val","vzip","SECRET","USERNAME","UserInfo","borrow","borrow_mut","clone","clone_into","eq","fmt","from","get_user_info_from_env_vars","into","ne","secret","to_owned","try_from","try_into","type_id","username","vzip"],"q":["zkp_client","","","","","","","","","","","","","zkp_client::auth","","","","","","zkp_client::seed","","","","","","","","","","","","","","","","","zkp_client::user","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","Uses <code>TCPStream::connect</code> to establish connection to server …","Env Logger level <code>info</code>. Set <code>$RUST_LOG</code> env variable …","create_commitsregister_user_with_serverprove authenticity …","","Prints any errors that occur during env variables …","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Random u32 number","","","","","","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2],"f":[null,null,null,null,null,null,[[["str",0]],["result",4,[["tcpstream",3],["error",3]]]],[[]],[[["userinfo",3],["str",0],["callback",6]],["result",4,[["error",3]]]],[[["str",0],["userinfo",3]],["result",4,[["error",3]]]],[[["vec",3,[["error",3]]]]],null,null,[[["tcpstream",3],["username",6]],["result",4,[["serverresponse",4],["error",3]]]],[[["tcpstream",3],["username",6]],["result",4,[["serverresponse",4],["error",3]]]],[[["seed",3],["u32",0]],["commits",3]],[[["tcpstream",3],["clientrequest",4]],["result",4,[["serverresponse",4],["error",3]]]],[[["tcpstream",3],["username",6],["answer",6]],["result",4,[["serverresponse",4],["error",3]]]],[[["tcpstream",3],["username",6],["commits",3]],["result",4,[["serverresponse",4],["error",3]]]],null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["seed",3]],[[["",0],["",0]]],[[["",0],["seed",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0],["seed",3]],["bool",0]],[[]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,[[]],null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["userinfo",3]],[[["",0],["",0]]],[[["",0],["userinfo",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[]],[[],["result",4,[["userinfo",3],["vec",3,[["error",3]]]]]],[[]],[[["",0],["userinfo",3]],["bool",0]],null,[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,[[]]],"p":[[3,"Seed"],[3,"UserInfo"]]},\
"zkp_common":{"doc":"zkp_common includes public constants <code>G</code> and <code>H</code> and data …","t":[17,17,0,0,5,6,13,13,4,3,13,13,6,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,13,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["G","H","request_dto","response_dto","write_and_flush_stream","Answer","Authenticate","CheckStatus","ClientRequest","Commits","ProveAuthentication","Register","Username","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","deserialize","deserialize","fmt","fmt","from","from","into","into","r1","r2","serialize","serialize","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","y1","y2","0","0","0","0","1","1","Challenge","Failure","ServerResponse","Success","borrow","borrow_mut","clone","clone_into","deserialize","fmt","from","into","serialize","to_owned","try_from","try_into","type_id","0","0"],"q":["zkp_common","","","","","zkp_common::request_dto","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","zkp_common::request_dto::ClientRequest","","","","","","zkp_common::response_dto","","","","","","","","","","","","","","","","","zkp_common::response_dto::ServerResponse",""],"d":["","","","","","","","","","r1, r2 from client","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","",""],"i":[0,0,0,0,0,0,1,1,0,0,1,1,0,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,2,2,1,2,1,2,1,2,1,2,1,2,2,3,4,5,6,3,5,7,7,0,7,7,7,7,7,7,7,7,7,7,7,7,7,7,8,9],"f":[null,null,null,null,[[["tcpstream",3],["serialize",8]],["result",4,[["error",3]]]],null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["commits",3]],[[["",0]],["clientrequest",4]],[[["",0],["",0]]],[[["",0],["",0]]],[[],["result",4]],[[],["result",4]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],null,null,[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["serverresponse",4]],[[["",0],["",0]]],[[],["result",4]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0]],["result",4]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null],"p":[[4,"ClientRequest"],[3,"Commits"],[13,"Register"],[13,"Authenticate"],[13,"ProveAuthentication"],[13,"CheckStatus"],[4,"ServerResponse"],[13,"Challenge"],[13,"Failure"]]},\
"zkp_server":{"doc":"zkp_server includes modules and functions that <code>zkp_client</code> …","t":[17,0,0,5,5,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,5,3,3,11,11,11,11,12,12,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,12,12,11,11],"n":["MAX_CHALLENGE_VAL","challenge","handlers","init_logger","init_session_store","session_store","Challenge","borrow","borrow_mut","clone","clone_into","deserialize","eq","fmt","from","into","ne","new","serialize","to_owned","try_from","try_into","type_id","val","vzip","handle_request","SessionStore","User","borrow","borrow","borrow_mut","borrow_mut","challenge","commits","fmt","from","from","into","into","is_verified","last_login","last_verified","new","register","remove","try_from","try_from","try_into","try_into","type_id","type_id","username","users","vzip","vzip"],"q":["zkp_server","","","","","","zkp_server::challenge","","","","","","","","","","","","","","","","","","","zkp_server::handlers","zkp_server::session_store","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Random u128 number","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,2,3,2,3,3,3,3,2,3,2,3,3,3,3,2,2,2,2,3,2,3,2,3,3,2,2,3],"f":[null,null,null,[[]],[[],["sessionstore",3]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["challenge",3]],[[["",0],["",0]]],[[],["result",4]],[[["",0],["challenge",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["",0],["challenge",3]],["bool",0]],[[]],[[["",0]],["result",4]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,[[]],[[["clientrequest",4],["sessionstore",3],["tcpstream",3]],["result",4,[["error",3]]]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],null,null,[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],null,null,null,[[]],[[["",0],["username",6],["commits",3]]],[[["",0],["username",6]],["option",4,[["user",3]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,[[]],[[]]],"p":[[3,"Challenge"],[3,"SessionStore"],[3,"User"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};